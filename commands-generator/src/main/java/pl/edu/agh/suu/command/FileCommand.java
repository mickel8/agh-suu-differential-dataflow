package pl.edu.agh.suu.command;

public class FileCommand implements Command {
    public final String filename;
    public final Integer batchSize;

    public FileCommand(String filename, Integer batchSize) {
        this.filename = filename;
        this.batchSize = batchSize;
    }

    @Override
    public String format() {
        return String.format("f %s %d", filename, batchSize);
    }
}
