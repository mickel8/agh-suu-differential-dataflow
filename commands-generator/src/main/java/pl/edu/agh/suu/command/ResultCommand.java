package pl.edu.agh.suu.command;


public class ResultCommand implements Command {
    public final Integer time;

    public ResultCommand(Integer time) {
        this.time = time;
    }

    @Override
    public String format() {
        return String.format("= %d", time);
    }
}
