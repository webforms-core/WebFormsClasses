// WebForms.java 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

package webformscore;

// WebForms Place Criteria (WPC) DSL
public class InputPlace {
    public static final String DOCUMENT = ",";
    public static final String WINDOW = "`";
    // When Calling TransientDOM, Using Root will Result in the Selection of the Transient Tag.
    public static final String ROOT = "~";
    public static final String HTML = ".";
    public static final String HEAD = "^";
    public static final String SCREEN_ORIENTATION = "%";
    public static final String ALL = "*";
    public static final String PARENT = "/";
    public static final String CURRENT = "$";
    public static final String TARGET = "!";
    public static final String UPPER = "-";

    public static String id(String id) {
        return id;
    }

    public static String name(String name) {
        return '(' + name + ')';
    }

    public static String name(String name, int index) {
        return '(' + name + ')' + index;
    }

    public static String allNames(String name) {
        return "(" + name + ")*";
    }

    public static String tag(String tag) {
        return '<' + tag + '>';
    }

    public static String tag(String tag, int index) {
        return '<' + tag + '>' + index;
    }

    public static String allTags(String tag) {
        return "<" + tag + ">*";
    }

    public static String child() {
        return "<>";
    }

    public static String child(int index) {
        return "<>" + index;
    }

    public static String allChild() {
        return "<>*";
    }

    public static String cssClass(String className) {
        return '{' + className + '}';
    }

    public static String cssClass(String className, int index) {
        return '{' + className + '}' + index;
    }

    public static String allClasses(String className) {
        return "{" + className + "}*";
    }

    public static String attribute(String name) {
        return '"' + name + '"';
    }

    public static String attribute(String name, int index) {
        return '"' + name + '"' + index;
    }

    public static String allAttributes(String name) {
        return "\"" + name + "\"*";
    }

    // Operator: '^', '$', '*', '~'
    public static String attribute(String name, String value, char operator) {
        return '"' + name + (operator != '\0' ? String.valueOf(operator) : "") + "'" + value + '"';
    }

    public static String attribute(String name, String value) {
        return attribute(name, value, '\0');
    }

    public static String attribute(String name, String value, int index, char operator) {
        return '"' + name + (operator != '\0' ? String.valueOf(operator) : "") + "'" + value + '"' + index;
    }

    public static String attribute(String name, String value, int index) {
        return attribute(name, value, index, '\0');
    }

    public static String allAttributes(String name, String value, char operator) {
        return "\"" + name + (operator != '\0' ? String.valueOf(operator) : "") + "'" + value + "\"*";
    }

    public static String allAttributes(String name, String value) {
        return allAttributes(name, value, '\0');
    }

    public static String query(String query) {
        return "*" + query.replace("=", "$[eq];").replace("|", "$[vb];").replace("?", "$[qu];");
    }

    public static String queryAll(String query) {
        return "[" + query.replace("=", "$[eq];").replace("|", "$[vb];").replace("?", "$[qu];");
    }
}
