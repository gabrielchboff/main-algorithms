import java.util.Scanner;

public class Main {
    public static void main(String[] args) {

        Scanner scanner = new Scanner(System.in);  // Move the scanner initialization outside the loop
        TodoList todoList = new TodoList("My Todo List");

        while (true) {
            System.out.println("Welcome to Java TODO List!");
            System.out.println("Version 0.1");
            System.out.println("""
                    ____________________________________
                        What would you like to do?
                        1. Show all tasks
                        2. Add a task
                        3. Remove a task
                        4. Complete a task
                        5. Incomplete a task
                        6. Edit a task
                        7. Show completed tasks
                        8. Exit
                    ___________________________________
                    """);

            System.out.print("Enter your choice: ");
            int choice = scanner.nextInt();  // Read user choice

            switch (choice) {
                case 1:
                    todoList.printTasks();
                    break;
                case 2:
                    System.out.println("Enter task name: ");
                    String name1 = scanner.next();
                    System.out.println("Enter task description: ");
                    String description1 = scanner.next();
                    Task task = new Task(name1, description1, false);
                    todoList.addTask(task);
                    System.out.println("Task added successfully!");
                    break;
                case 3:
                    System.out.println("Enter task name: ");
                    String name2 = scanner.next();
                    todoList.tasks.removeIf(t -> t.getName().equals(name2));
                    break;
                case 4:
                    // complete a task
                    System.out.println("Enter task name: ");
                    String name3 = scanner.next();
                    for (Task t : todoList.tasks) {
                        if (t.getName().equals(name3)) {
                            todoList.completeTask(t);
                        }
                    }
                    break;
                case 5:
                    // incomplete a task
                    System.out.println("Enter task name: ");
                    String name4 = scanner.next();
                    for (Task t : todoList.tasks) {
                        if (t.getName().equals(name4)) {
                            todoList.incompleteTask(t);
                        }
                    }
                    break;
                case 6:
                    // edit a task
                    System.out.println("Enter task name to edit: ");
                    String name5 = scanner.next();
                    System.out.println("Enter new task name: ");
                    String nameEdited = scanner.next();
                    System.out.println("Enter the new task description: ");
                    String description3 = scanner.next();
                    for (Task t : todoList.tasks) {
                        if (t.getName().equals(name5)) {
                            todoList.editTask(t, nameEdited, description3);
                        }
                    }
                    break;
                case 7:
                    // show completed tasks
                    todoList.printCompletedTasks();
                    break;
                case 8:
                    System.out.println("Goodbye!");
                    scanner.close();  // Close the scanner when the program is done
                    System.exit(0);
                    break;
                default:
                    System.out.println("Invalid choice");
                    scanner.close();  // Close the scanner in case of invalid choice
                    System.exit(1);
                    break;
            }
        }
    }
}
