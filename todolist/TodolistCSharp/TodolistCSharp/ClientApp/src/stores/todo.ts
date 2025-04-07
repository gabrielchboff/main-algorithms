import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Todo } from '../types/todo'

export const useTodoStore = defineStore('todo', () => {
  const todos = ref<Todo[]>([])
  const isLoading = ref(false)

  const fetchTodos = async () => {
    isLoading.value = true
    try {
      const response = await fetch('http://localhost:5000/api/todo')
      todos.value = await response.json() as Todo[]
    } catch (error) {
      console.error('Error fetching todos:', error)
    } finally {
      isLoading.value = false
    }
  }

  const addTodo = async (todo: Omit<Todo, 'id' | 'createdAt' | 'completedAt'>) => {
    try {
      const response = await fetch('http://localhost:5000/api/todo', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(todo),
      })
      const newTodo = await response.json() as Todo
      todos.value.push(newTodo)
    } catch (error) {
      console.error('Error adding todo:', error)
    }
  }

  const toggleTodo = async (todo: Todo) => {
    try {
      await fetch(`http://localhost:5000/api/todo/${todo.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          ...todo,
          isCompleted: !todo.isCompleted,
        }),
      })
      const index = todos.value.findIndex((t) => t.id === todo.id)
      if (index !== -1) {
        todos.value[index] = {
          ...todo,
          isCompleted: !todo.isCompleted,
          completedAt: !todo.isCompleted ? new Date().toISOString() : null,
        }
      }
    } catch (error) {
      console.error('Error updating todo:', error)
    }
  }

  const deleteTodo = async (id: number) => {
    try {
      await fetch(`http://localhost:5000/api/todo/${id}`, {
        method: 'DELETE',
      })
      todos.value = todos.value.filter((todo) => todo.id !== id)
    } catch (error) {
      console.error('Error deleting todo:', error)
    }
  }

  return {
    todos,
    isLoading,
    fetchTodos,
    addTodo,
    toggleTodo,
    deleteTodo,
  }
})
