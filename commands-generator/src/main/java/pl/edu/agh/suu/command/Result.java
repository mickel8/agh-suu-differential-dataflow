package pl.edu.agh.suu.command;


public class Result implements Command {
    public final Integer time;

    public Result(Integer time) {
        this.time = time;
    }

    @Override
    public String format() {
        return String.format("= %d", time);
    }
}
