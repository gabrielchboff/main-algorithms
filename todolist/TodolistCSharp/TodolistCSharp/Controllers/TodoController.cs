using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Linq;
using TodolistCSharp.Models;

namespace TodolistCSharp.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class TodoController : ControllerBase
    {
        private static List<Todo> _todos = new List<Todo>();

        [HttpGet]
        public ActionResult<IEnumerable<Todo>> GetAll()
        {
            return Ok(_todos);
        }

        [HttpGet("{id}")]
        public ActionResult<Todo> Get(int id)
        {
            var todo = _todos.FirstOrDefault(t => t.Id == id);
            if (todo == null)
                return NotFound();
            
            return Ok(todo);
        }

        [HttpPost]
        public ActionResult<Todo> Create(Todo todo)
        {
            todo.Id = _todos.Count > 0 ? _todos.Max(t => t.Id) + 1 : 1;
            todo.CreatedAt = DateTime.UtcNow;
            todo.IsCompleted = false;
            
            _todos.Add(todo);
            return CreatedAtAction(nameof(Get), new { id = todo.Id }, todo);
        }

        [HttpPut("{id}")]
        public IActionResult Update(int id, Todo todo)
        {
            var existingTodo = _todos.FirstOrDefault(t => t.Id == id);
            if (existingTodo == null)
                return NotFound();

            existingTodo.Title = todo.Title;
            existingTodo.Description = todo.Description;
            existingTodo.IsCompleted = todo.IsCompleted;
            existingTodo.CompletedAt = todo.IsCompleted ? DateTime.UtcNow : null;

            return NoContent();
        }

        [HttpDelete("{id}")]
        public IActionResult Delete(int id)
        {
            var todo = _todos.FirstOrDefault(t => t.Id == id);
            if (todo == null)
                return NotFound();

            _todos.Remove(todo);
            return NoContent();
        }
    }
}
