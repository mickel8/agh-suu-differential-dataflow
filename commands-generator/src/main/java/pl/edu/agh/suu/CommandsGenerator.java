package pl.edu.agh.suu;

import com.martiansoftware.jsap.*;
import it.unimi.dsi.webgraph.BVGraph;
import it.unimi.dsi.webgraph.ImmutableGraph;
import it.unimi.dsi.webgraph.LazyIntIterator;
import it.unimi.dsi.webgraph.NodeIterator;

import java.io.IOException;
import java.lang.reflect.InvocationTargetException;
import java.nio.file.Path;

public class CommandsGenerator {

    private static final String DATASET = "twitter-2010";

    public static void main(String[] args) throws JSAPException {
        SimpleJSAP jsap = new SimpleJSAP(
                "Twitter DataBroker",
                "Access Twitter dataset files and print nodes with their successors",
                new Parameter[] {
                        new FlaggedOption( "path", JSAP.STRING_PARSER, JSAP.NO_DEFAULT, JSAP.REQUIRED, 'p', "path",
                                "Path to dataset files." ),
                }
        );

        JSAPResult config = jsap.parse(args);
        if (jsap.messagePrinted()) {
            System.exit( 1 );
        }

        Path path = Path.of(config.getString("path"), DATASET);
        try {
            generateOffsets(path);
            printDD(path);
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

    public static void printDD(Path path) throws IOException {
        ImmutableGraph graph = ImmutableGraph.load(path.toString());
        System.out.println(graph.numNodes());

        NodeIterator nodes = graph.nodeIterator();
        printNodes(nodes);
    }

    public static void printNodes(NodeIterator nodes) {
        int debugCounter = 100;
        int nodeOrdinal;
        while ((nodeOrdinal = nodes.nextInt()) != -1) {
            System.out.println(nodeOrdinal);
            LazyIntIterator successors = nodes.successors();
            printSuccessors(successors, nodeOrdinal);
            if ((debugCounter -= 1) == 0) {
                break;
            }
        }
    }

    public static void printSuccessors(LazyIntIterator successors, int nodeOrdinal) {
        int debugCounter = 100;
        int successorOrdinal;
        while ((successorOrdinal = successors.nextInt()) != -1) {
            System.out.println(String.format("%d %d", nodeOrdinal, successorOrdinal));
            if ((debugCounter -= 1) == 0) {
                break;
            }
        }
    }
}
