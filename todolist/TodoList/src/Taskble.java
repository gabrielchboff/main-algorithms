public interface Taskble {
    int id = 0;
    String name = "";
    String description = "";
    boolean status = false;

    public void setStatus(boolean status);
    public void setName(String name);
    public void setDescription(String description);

    public boolean getStatus();
    public String getName();
    public String getDescription();

    public void printTask();
}
