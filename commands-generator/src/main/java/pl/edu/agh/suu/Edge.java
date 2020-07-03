package pl.edu.agh.suu;

public class Edge<T> {
    public final T from;
    public final T to;

    public Edge(T from, T to) {
        this.from = from;
        this.to = to;
    }
}
