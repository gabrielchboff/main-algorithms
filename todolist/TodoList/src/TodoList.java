import java.util.ArrayList;
import java.util.List;

public class TodoList  {
    String name = "";
    List<Task> tasks = new ArrayList<>();

    public TodoList(String name) {
        this.name = name;
        List<Task> tasks = new ArrayList<>();
    }

    public void addTask(Task task) {
        tasks.add(task);
    }

    public void printTasks() {
        for (Task task : tasks) {
            task.printTask();
        }
    }

    public void removeTask(Task task) {
        tasks.remove(task);
    }

    public void completeTask(Task task) {
        task.setStatus(true);
    }

    public void incompleteTask(Task task) {
        task.setStatus(false);
    }

    public void editTask(Task task, String name, String description) {
        for (Task t : tasks) {
            if (t.getName().equals(task.getName())) {
                t.setName(name);
                t.setDescription(description);
            }
        }
    }

    public void printCompletedTasks() {
        for (Task task : tasks) {
            if (task.getStatus()) {
                task.printTask();
            }
        }
    }

}
