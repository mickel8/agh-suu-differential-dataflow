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
import java.util.stream.Stream;

public class CommandsGenerator {

    private static final String DATASET = "twitter-2010";
    private static final String BENCHMARK_FILENAME = "benchmark";
    private static final String LOAD_FILENAME = "load-graph";
    private static final String TEST_FILENAME = "test-streaming";

    private static final int CLIENT_BATCH_SIZE = 30;

    public static void main(String[] args) throws JSAPException {
        SimpleJSAP jsap = new SimpleJSAP(
                "Commands Genarator for Tegra-like DD streaming benchmark",
                "Access Twitter dataset files to generate benchmark commands",
                new Parameter[] {
                        new FlaggedOption( "path", JSAP.STRING_PARSER, JSAP.NO_DEFAULT, JSAP.REQUIRED, 'p', "path",
                                "Path to dataset files" ),
                        new FlaggedOption( "steps", JSAP.INTEGER_PARSER, "5", JSAP.REQUIRED, 'n', JSAP.NO_LONGFLAG,
                                "The number of times to compute graph" ),
                        new FlaggedOption("percent", JSAP.INTEGER_PARSER, "100", JSAP.REQUIRED, JSAP.NO_SHORTFLAG, "percent",
                                "The percent of graph to process")
                }
        );

        JSAPResult config = jsap.parse(args);
        if (jsap.messagePrinted()) {
            System.exit(1);
        }

        Path path = Path.of(config.getString("path"), DATASET);
        int steps = config.getInt("steps");
        int percent = config.getInt("percent");
        try {
            generateOffsets(path);
            ImmutableGraph graph = ImmutableGraph.load(path.toString());

            Path file = Path.of(BENCHMARK_FILENAME);
            try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
                Stream.of(
                        generateGraphLoadCommands(graph, percent, 0),
                        generateTestPurelyStreamingAnalysisCommands(graph, percent, steps, 1))
                    .map(Command::format)
                    .forEach(writer::println);
            }
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

    public static FileCommand generateGraphLoadCommands(ImmutableGraph graph, int percent, int time) throws IOException {
        String filename = String.format("%s-%dp", LOAD_FILENAME, percent);
        Path file = Path.of(filename);
        long edgesQuantity = graph.numArcs() / 100 * percent;

        System.out.printf("Generating %s, %d edges%n", filename, edgesQuantity);

        try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
            NodeIterator nodes = graph.nodeIterator();
            int nodeOrdinal;

            while ((nodeOrdinal = nodes.nextInt()) != -1 && edgesQuantity > 0) {
                LazyIntIterator successors = nodes.successors();
                int successorOrdinal;
                while ((successorOrdinal = successors.nextInt()) != -1 && edgesQuantity > 0) {

                    Edge<Integer> edge = new Edge<>(nodeOrdinal, successorOrdinal);
                    writer.println(new AddCommand(edge, time).format());

                    edgesQuantity--;
                }
                System.out.printf("%d edges...%n", edgesQuantity);
            }
            writer.println(new ResultCommand(time + 1).format());
        }

        return new FileCommand(filename, CLIENT_BATCH_SIZE);
    }

    public static FileCommand generateTestPurelyStreamingAnalysisCommands(ImmutableGraph graph, int percent, int steps, int time) throws IOException {
        String filename = String.format("%s-%dp-%ds", TEST_FILENAME, percent, steps);
        Path file = Path.of(filename);
        long edgesQuantity = (graph.numArcs() / 100 * percent) / 1000;

        int repetitions = 1000 * percent / 100;
        int batch = repetitions / steps;

        System.out.printf("Generating %s, %d edges%n", filename, edgesQuantity * repetitions);

        try (PrintWriter writer = new PrintWriter(Files.newBufferedWriter(file))) {
            writer.println(new MeasureCommand().format());
            IntStream.range(0, steps)
                    .forEach(step -> {
                        IntStream.range(0, batch)
                                .forEach(i -> generateAddRemoveCommands(writer, graph, edgesQuantity, step + time));
                        writer.println(new ResultCommand(step + time + 1).format());
                        writer.println(new MeasureCommand().format());
                        System.out.printf("%d step...%n", step);
                    });
        }

        return new FileCommand(filename, CLIENT_BATCH_SIZE);
    }

    public static void generateAddRemoveCommands(PrintWriter writer, ImmutableGraph graph, long edgesQuantity, int time) {
        List<Edge<Integer>> edges = generateRandomEdges(graph, edgesQuantity);
        edges.stream()
                .map(edge -> new AddCommand(edge, time))
                .map(Command::format)
                .forEach(writer::println);
        edges.stream()
                .map(edge -> new RemoveCommand(edge, time))
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
