public class Task implements Taskble {
    private String name;
    private String description;
    private boolean status;

    public Task(String name, String description, boolean status) {
        this.name = name;
        this.description = description;
        this.status = status;
    }

    public String getName() {
        return name;
    }

    public String getDescription() {
        return description;
    }

    public boolean getStatus() {
        return status;
    }

    public void setName(String name) {
        this.name = name;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public void setStatus(boolean status) {
        this.status = status;
    }

    public void printTask() {
        System.out.println("Name: " + name);
        System.out.println("Description: " + description);
        System.out.println("Status: " + status);
    }


}
