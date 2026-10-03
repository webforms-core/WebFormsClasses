// WebForms.java 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

package webformscore;

public class ExtensionWebFormsMethods {
    public static String child(String text, String value) {
        if (text == null || text.length() < 1) {
            return value;
        }
        return text + "|" + value;
    }

    public static String parent(String text) {
        if (text == null || text.length() < 1) {
            return text;
        }
        if (text.endsWith("|/") || text.endsWith("//")) {
            return text + '/';
        }
        return text + "|/";
    }

    public static String criteria(String text, String value) {
        if (text == null || text.length() < 1) {
            return value;
        }
        return text + "?" + value.replace("|", "$[vb];").replace("?", "$[qu];").replace("=", "$[eq];");
    }

    public static String appendFetchReplace(String text, String searchValue, String value) {
        char fs = (char) 28;
        text = text.length() > 0 ? text.substring(1) : "";
        return "@;" + searchValue + fs + value + fs + text;
    }

    public static String lineBreak(String text, boolean encodeLine) {
        String encode = encodeLine ? "$[sln];" : "";
        return text.replace("\r\n", encode).replace("\n", encode).replace("\r", encode);
    }

    public static String lineBreak(String text) {
        return lineBreak(text, false);
    }

    // Converts Numbers to Strings
    public static String toJsString(String text) {
        return "\"" + text + "\"";
    }

    // Get JS Object Momentary 
    public static String toJsObject(String text) {
        return "$" + text;
    }

    // Get JS Object Returned Value Once
    public static String toJsReturnObject(String text) {
        return "$@" + text;
    }
}
