package pl.edu.agh.suu;

import com.martiansoftware.jsap.*;
import it.unimi.dsi.webgraph.BVGraph;
import it.unimi.dsi.webgraph.ImmutableGraph;
import it.unimi.dsi.webgraph.LazyIntIterator;
import it.unimi.dsi.webgraph.NodeIterator;
import pl.edu.agh.suu.command.*;

import java.io.IOException;
import java.io.PrintWriter;
import java.lang.reflect.InvocationTargetException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

public class CommandsGenerator {

    private static final String DATASET = "twitter-2010";
    private static final int PERCENT = 1;

    public static void main(String[] args) throws JSAPException {
        SimpleJSAP jsap = new SimpleJSAP(
                "Commands Genarator for Tegra-like DD benchmarks",
                "Access Twitter dataset files to generate benchmark commands",
                new Parameter[] {
                        new FlaggedOption( "path", JSAP.STRING_PARSER, JSAP.NO_DEFAULT, JSAP.REQUIRED, 'p', "path",
                                "Path to dataset files" ),
                }
        );

        JSAPResult config = jsap.parse(args);
        if (jsap.messagePrinted()) {
            System.exit(1);
        }

        Path path = Path.of(config.getString("path"), DATASET);
        try {
            generateOffsets(path);
            ImmutableGraph graph = ImmutableGraph.load(path.toString());

            generateGraphLoadCommands(graph, 0);
            generateTestUpdateThroughputCommands(graph, 1);
            generateTestSnapshotRetrievalLatencyCommands(graph, 1);
            generateTestPurelyStreamingAnalysisCommands(graph, 1);

        } catch (IOException | NoSuchMethodException | InstantiationException | InvocationTargetException |
                JSAPException | IllegalAccessException | ClassNotFoundException e) {
            e.printStackTrace();
        }
    }

    public static void generateOffsets(Path path) throws IOException, IllegalAccessException, JSAPException,
            InstantiationException, NoSuchMethodException, InvocationTargetException, ClassNotFoundException {
        String[] arguments = new String[]{"-o", "-O", "-L", path.toString()};
        BVGraph.main(arguments);
    }

    public static void generateGraphLoadCommands(ImmutableGraph graph, int time) {
        Path file = Path.of("load-graph.cmds");
        long edgesQuantity = graph.numArcs() / 100 * PERCENT;

        System.out.printf("Generating load-graph, %d edges%n", edgesQuantity);

        try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
            NodeIterator nodes = graph.nodeIterator();
            int nodeOrdinal;

            while ((nodeOrdinal = nodes.nextInt()) != -1 && edgesQuantity > 0) {
                LazyIntIterator successors = nodes.successors();
                int successorOrdinal;
                while ((successorOrdinal = successors.nextInt()) != -1 && edgesQuantity > 0) {

                    Edge<Integer> edge = new Edge<>(nodeOrdinal, successorOrdinal);
                    writer.println(new Add(edge, time).format());

                    edgesQuantity--;
                }
                System.out.printf("%d edges...%n", edgesQuantity);
            }
            writer.println(new Result(time + 1).format());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void generateTestUpdateThroughputCommands(ImmutableGraph graph, int time) {
        Path file = Path.of("test-throughput.cmds");
        long edgesQuantity = 1000000 / 100 * PERCENT;

        System.out.printf("Generating test-throughput, %d edges%n", edgesQuantity);

        try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
            writer.println(new Measure().format());
            generateAddRemoveCommands(writer, graph, edgesQuantity, time);
            writer.println(new Measure().format());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void generateTestSnapshotRetrievalLatencyCommands(ImmutableGraph graph, int time) {
        Path file = Path.of("test-retrieval.cmds");
        long edgesQuantity = 1000000 / 100 * PERCENT;
        int repetitions = 1000 / 100 * PERCENT;

        System.out.printf("Generating test-throughput, %d edges%n", edgesQuantity * repetitions);

        try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
            writer.println(new Measure().format());
            IntStream.range(0, repetitions)
                    .forEach(i -> generateAddRemoveCommands(writer, graph, edgesQuantity, time));
            writer.println(new Result(time + 1).format());
            writer.println(new Measure().format());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void generateTestPurelyStreamingAnalysisCommands(ImmutableGraph graph, int time) {
        Path file = Path.of("test-streaming.cmds");
        long edgesQuantity = (graph.numArcs() / 100 * PERCENT) / 1000;

        int repetitions = 1000 / 100 * PERCENT;
        int steps = 5;
        int batch = repetitions / steps;

        System.out.printf("Generating test-streaming, %d edges%n", edgesQuantity * repetitions);

        try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
            writer.println(new Measure().format());
            IntStream.range(0, steps)
                    .forEach(step -> {
                        IntStream.range(0, batch)
                                .forEach(i -> generateAddRemoveCommands(writer, graph, edgesQuantity, step + time));
                        writer.println(new Result(step + time + 1).format());
                        writer.println(new Measure().format());
                        System.out.printf("%d step...%n", step);
                    });
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void generateAddRemoveCommands(PrintWriter writer, ImmutableGraph graph, long edgesQuantity, int time) {
        List<Edge<Integer>> edges = generateRandomEdges(graph, edgesQuantity);
        edges.stream()
                .map(edge -> new Add(edge, time))
                .map(Command::format)
                .forEach(writer::println);
        edges.stream()
                .map(edge -> new Remove(edge, time))
                .map(Command::format)
                .forEach(writer::println);
    }

    public static List<Edge<Integer>> generateRandomEdges(ImmutableGraph graph, long quantity) {
        Random generator = new Random();
        int maxOrdinal = graph.numNodes();

        return LongStream
                .range(0, quantity)
                .mapToObj(i -> {
                    int to = generator.nextInt(maxOrdinal);
                    int from = generator.nextInt(maxOrdinal);
                    return new Edge<>(from, to);
                })
                .collect(Collectors.toList());
    }
}
