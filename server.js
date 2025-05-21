const fs = require('fs'); // For file system operations in Node.js
const path = require('path'); // For robust path handling

const DATA_FILE = "mcp_data.json";

/**
 * Loads data from the JSON file.
 * Assumes Node.js environment.
 * @returns {object} The loaded data, or { tasks: {} } on error or if file doesn't exist.
 */
function loadData() {
    if (fs.existsSync(DATA_FILE)) {
        try {
            const fileContent = fs.readFileSync(DATA_FILE, 'utf8');
            return JSON.parse(fileContent);
        } catch (error) {
            console.error("Error loading or parsing data:", error);
            return { tasks: {} }; // Default structure on error
        }
    }
    return { tasks: {} }; // Default structure if file doesn't exist
}

/**
 * Saves data to the JSON file.
 * Assumes Node.js environment.
 * @param {object} data - The data object to save.
 */
function saveData(data) {
    try {
        const jsonData = JSON.stringify(data, null, 4); // Pretty print JSON
        fs.writeFileSync(DATA_FILE, jsonData, 'utf8');
    } catch (error) {
        console.error("Error saving data:", error);
    }
}

/**
 * Generates the next available ID for an object (dictionary).
 * Assumes keys are or can be converted to numbers.
 * @param {object} dataObject - The object to generate an ID for.
 * @returns {number} The next available ID.
 */
function nextId(dataObject) {
    if (!dataObject || Object.keys(dataObject).length === 0) {
        return 1;
    }
    const maxId = Object.keys(dataObject)
        .map(key => parseInt(key, 10))
        .filter(num => !isNaN(num)) // Ensure only valid numbers
        .reduce((max, current) => Math.max(max, current), 0); // Start with 0
    return maxId + 1;
}

/**
 * Adds a new task.
 * @param {object} tasks - The tasks object.
 * @param {string} name - The name of the new task.
 * @param {string|null} [parentId=null] - The ID of the parent task, if any.
 * @returns {[string|null, string]} A tuple containing the new task ID (or null on error) and a status message.
 */
function addTask(tasks, name, parentId = null) {
    const taskId = String(nextId(tasks));

    if (parentId && !tasks[String(parentId)]) {
        return [null, "Error: Parent task ID not found."];
    }

    tasks[taskId] = {
        id: taskId,
        name: name,
        parent_id: parentId ? String(parentId) : null,
        todos: {}, // id: { text: "...", done: false }
        notes: [], // array of strings
        status: "new", // new, blocked, in progress, finished
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
    };
    return [taskId, `Task '${name}' (ID: ${taskId}) created.`];
}

/**
 * Formats a single task for display.
 * @param {object} task - The task object to format.
 * @returns {string} A formatted string representation of the task.
 */
function formatTask(task) {
    const parentInfo = task.parent_id ? `(Parent: ${task.parent_id})` : "";
    return `- ID: ${task.id}, Name: ${task.name}, Status: ${task.status} ${parentInfo} (Created: ${task.created_at.substring(0, 10)})`;
}

/**
 * Lists all tasks or a specific task with its details.
 * @param {object} tasks - The tasks object.
 * @param {string|null} [taskId=null] - The ID of a specific task to list, or null to list all.
 * @returns {string} A string containing the list of tasks or task details.
 */
function listTasks(tasks, taskId = null) {
    if (!tasks || Object.keys(tasks).length === 0) {
        return "No tasks found.";
    }
    const output = [];
    if (taskId) {
        const task = tasks[String(taskId)];
        if (task) {
            output.push(formatTask(task));
            if (task.todos && Object.keys(task.todos).length > 0) {
                output.push("  Todos:");
                for (const todoId in task.todos) {
                    if (task.todos.hasOwnProperty(todoId)) {
                        const todo = task.todos[todoId];
                        const status = todo.done ? "✓" : "✗";
                        output.push(`    [${status}] (${todoId}) ${todo.text}`);
                    }
                }
            }
            if (task.notes && task.notes.length > 0) {
                output.push("  Notes:");
                task.notes.forEach((note, i) => {
                    output.push(`    - (${i + 1}) ${note}`);
                });
            }
        } else {
            return `Error: Task ID ${taskId} not found.`;
        }
    } else {
        output.push("Tasks:");
        Object.values(tasks)
            .sort((a, b) => new Date(a.created_at) - new Date(b.created_at)) // Sort by creation date
            .forEach(taskData => {
                output.push(formatTask(taskData));
            });
    }
    return output.join("\n");
}

/**
 * Adds a todo item to a specific task.
 * @param {object} tasks - The tasks object.
 * @param {string} taskId - The ID of the task to add the todo to.
 * @param {string} text - The text content of the todo.
 * @returns {string} A status message.
 */
function addTodo(tasks, taskId, text) {
    const strTaskId = String(taskId);
    if (!tasks[strTaskId]) {
        return "Error: Task ID not found.";
    }
    const task = tasks[strTaskId];
    if (!task.todos) task.todos = {}; // Ensure todos object exists
    const todoId = String(nextId(task.todos));
    task.todos[todoId] = { text: text, done: false };
    task.updated_at = new Date().toISOString();
    return `Todo '${text}' (ID: ${todoId}) added to task ${strTaskId}.`;
}

/**
 * Toggles the 'done' status of a todo item.
 * @param {object} tasks - The tasks object.
 * @param {string} taskId - The ID of the task containing the todo.
 * @param {string} todoId - The ID of the todo to toggle.
 * @returns {string} A status message.
 */
function toggleTodo(tasks, taskId, todoId) {
    const strTaskId = String(taskId);
    const strTodoId = String(todoId);

    if (!tasks[strTaskId]) {
        return "Error: Task ID not found.";
    }
    const task = tasks[strTaskId];
    if (!task.todos || !task.todos[strTodoId]) {
        return "Error: Todo ID not found in this task.";
    }
    task.todos[strTodoId].done = !task.todos[strTodoId].done;
    task.updated_at = new Date().toISOString();
    const status = task.todos[strTodoId].done ? "done" : "not done";
    return `Todo ${strTodoId} in task ${strTaskId} marked as ${status}.`;
}

/**
 * Adds a note to a specific task.
 * @param {object} tasks - The tasks object.
 * @param {string} taskId - The ID of the task to add the note to.
 * @param {string} noteText - The text content of the note.
 * @returns {string} A status message.
 */
function addNote(tasks, taskId, noteText) {
    const strTaskId = String(taskId);
    if (!tasks[strTaskId]) {
        return "Error: Task ID not found.";
    }
    if (!tasks[strTaskId].notes) { // Ensure notes array exists
        tasks[strTaskId].notes = [];
    }
    tasks[strTaskId].notes.push(noteText);
    tasks[strTaskId].updated_at = new Date().toISOString();
    return `Note added to task ${strTaskId}.`;
}

/**
 * Fetches and formats notes for a specific task.
 * @param {object} tasks - The tasks object.
 * @param {string} taskId - The ID of the task to get notes for.
 * @returns {string} A string containing the formatted notes or an error/status message.
 */
function getNotes(tasks, taskId) {
    const strTaskId = String(taskId);
    if (!tasks[strTaskId]) {
        return "Error: Task ID not found.";
    }
    const taskNotes = tasks[strTaskId].notes || [];
    if (taskNotes.length === 0) {
        return `No notes found for task ${strTaskId}.`;
    }
    const output = [`Notes for Task ${strTaskId} ('${tasks[strTaskId].name}'):`];
    taskNotes.forEach((note, i) => {
        output.push(`  (${i + 1}) ${note}`);
    });
    return output.join("\n");
}

/**
 * Sets the status of a task.
 * @param {object} tasks - The tasks object.
 * @param {string} taskId - The ID of the task to update.
 * @param {string} status - The new status for the task.
 * @returns {string} A status message.
 */
function setStatus(tasks, taskId, status) {
    const strTaskId = String(taskId);
    const validStatuses = ["new", "blocked", "in progress", "finished"];
    if (!validStatuses.includes(status)) {
        return `Error: Invalid status. Choose from: ${validStatuses.join(", ")}.`;
    }
    if (!tasks[strTaskId]) {
        return "Error: Task ID not found.";
    }
    tasks[strTaskId].status = status;
    tasks[strTaskId].updated_at = new Date().toISOString();
    return `Status of task ${strTaskId} set to '${status}'.`;
}

/**
 * Links a task to a parent task.
 * @param {object} tasks - The tasks object.
 * @param {string} taskId - The ID of the task to be linked.
 * @param {string} parentId - The ID of the parent task.
 * @returns {string} A status message.
 */
function linkTask(tasks, taskId, parentId) {
    const strTaskId = String(taskId);
    const strParentId = String(parentId);

    if (!tasks[strTaskId]) {
        return `Error: Task ID ${strTaskId} not found.`;
    }
    if (!tasks[strParentId]) {
        return `Error: Parent task ID ${strParentId} not found.`;
    }
    if (strTaskId === strParentId) {
        return "Error: Cannot link a task to itself.";
    }

    // Basic circular dependency check
    let current = tasks[strParentId];
    const visitedDuringCheck = new Set(); // To prevent infinite loops during this check only

    while (current && current.parent_id) {
        if (visitedDuringCheck.has(current.id)) {
             // This implies a loop in the existing parent chain, not necessarily caused by the new link yet,
             // but it makes further checking unreliable.
            return "Error: Circular dependency or broken parent chain detected during check.";
        }
        visitedDuringCheck.add(current.id);

        if (current.parent_id === strTaskId) {
            return `Error: Circular dependency detected. Cannot link ${strTaskId} to ${strParentId}.`;
        }
        if (!tasks[current.parent_id]) { // Parent in the chain doesn't exist
             break; // Stop checking if the chain is broken
        }
        current = tasks[current.parent_id];
    }

    tasks[strTaskId].parent_id = strParentId;
    tasks[strTaskId].updated_at = new Date().toISOString();
    return `Task ${strTaskId} linked to parent task ${strParentId}.`;
}

// To make these functions usable in other Node.js files, you can export them:
module.exports = {
    loadData,
    saveData,
    nextId,
    addTask,
    listTasks,
    formatTask,
    addTodo,
    toggleTodo,
    addNote,
    getNotes,
    setStatus,
    linkTask
};