<template>
  <div class="todo-list">
    <div class="add-todo">
      <input
        v-model="newTodo.title"
        type="text"
        placeholder="Todo title"
        class="input"
      />
      <input
        v-model="newTodo.description"
        type="text"
        placeholder="Todo description"
        class="input"
      />
      <button @click="handleAddTodo" class="btn btn-primary">Add Todo</button>
    </div>

    <div v-if="isLoading" class="loading">Loading...</div>

    <div v-else class="todos">
      <div v-for="todo in todos" :key="todo.id" class="todo-item">
        <div class="todo-content">
          <input
            type="checkbox"
            :checked="todo.isCompleted"
            @change="() => toggleTodo(todo)"
          />
          <div :class="{ completed: todo.isCompleted }">
            <h3>{{ todo.title }}</h3>
            <p>{{ todo.description }}</p>
            <small>Created: {{ new Date(todo.createdAt).toLocaleString() }}</small>
            <small v-if="todo.completedAt">
              Completed: {{ new Date(todo.completedAt).toLocaleString() }}
            </small>
          </div>
        </div>
        <button @click="() => deleteTodo(todo.id)" class="btn btn-danger">
          Delete
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive } from 'vue'
import { useTodoStore } from '../stores/todo'
import type { Todo } from '../types/todo'

const store = useTodoStore()
const { todos, isLoading, fetchTodos, addTodo, toggleTodo, deleteTodo } = store

const newTodo = reactive({
  title: '',
  description: '',
  isCompleted: false,
})

const handleAddTodo = async () => {
  if (newTodo.title.trim()) {
    await addTodo(newTodo)
    newTodo.title = ''
    newTodo.description = ''
  }
}

onMounted(() => {
  fetchTodos()
})
</script>

<style scoped>
.todo-list {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}

.add-todo {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}

.input {
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  flex: 1;
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary {
  background-color: #4CAF50;
  color: white;
}

.btn-danger {
  background-color: #f44336;
  color: white;
}

.todos {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.todo-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.todo-content {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}

.completed {
  text-decoration: line-through;
  color: #666;
}

.loading {
  text-align: center;
  padding: 2rem;
  color: #666;
}
</style>
