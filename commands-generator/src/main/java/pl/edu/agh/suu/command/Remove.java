package pl.edu.agh.suu.command;

import pl.edu.agh.suu.Edge;

public class Remove implements Command {
    public final Edge<Integer> edge;
    public final Integer time;

    public Remove(Edge<Integer> edge, Integer time) {
        this.edge = edge;
        this.time = time;
    }

    @Override
    public String format() {
        return String.format("- %d %d %d", edge.from, edge.to, time);
    }
}
