// WebForms.java 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

package webformscore;

// Do not Add any Data Before or After it
public class Fetch {
    private static final char RS = (char) 30;
    private static final char US = (char) 31;

    // Method
    public static String random(int maxValue) {
        return "@mr" + maxValue;
    }

    public static String random(int minValue, int maxValue) {
        return "@mr" + maxValue + RS + minValue;
    }

    public static String spaceToChar(String text, String character) {
        return "@sc" + character + RS + text;
    }

    public static String spaceToChar(String text) {
        return spaceToChar(text, "-");
    }

    public static String encodeUri(String text) {
        return "@ue" + text;
    }

    public static String decodeUri(String text) {
        return "@ud" + text;
    }

    public static String method(String methodName, Object[] args) {
        String returnValue = "@cm" + methodName;
        if (args != null) {
            returnValue += (args.length > 0) ? RS + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        return returnValue;
    }

    public static String method(String methodName) {
        return method(methodName, null);
    }

    public static String moduleMethod(String methodName, Object[] args) {
        String returnValue = "@cM" + methodName;
        if (args != null) {
            returnValue += (args.length > 0) ? RS + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        return returnValue;
    }

    public static String moduleMethod(String methodName) {
        return moduleMethod(methodName, null);
    }

    // MethodName: The Method Name May Need to Include the Class Name, Separated by a Period. Example: MyClassName.MyMethodName
    public static String wasmMethod(String wasmLanguage, String wasmUrl, String methodName, Object[] args) {
        String returnValue = "@wA" + wasmLanguage + RS + wasmUrl + RS + methodName;
        if (args != null) {
            returnValue += (args.length > 0) ? RS + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        return returnValue;
    }

    public static String wasmMethod(String wasmLanguage, String wasmUrl, String methodName) {
        return wasmMethod(wasmLanguage, wasmUrl, methodName, null);
    }

    public static String script(String scriptText) {
        return "@_" + scriptText.replace("\n", "$[ln];");
    }

    public static String loadUrl(String url, boolean fetchScript) {
        return "@lu" + url + (fetchScript ? RS + "1" : "");
    }

    public static String loadUrl(String url) {
        return loadUrl(url, false);
    }

    public static String loadHtml(String url, String fetchInputPlace, boolean fetchScript) {
        return "@lh" + url + RS + (fetchScript ? "1" : "0") + ((fetchInputPlace != null && !fetchInputPlace.isEmpty()) ? RS + fetchInputPlace : "");
    }

    public static String loadHtml(String url, String fetchInputPlace) {
        return loadHtml(url, fetchInputPlace, false);
    }

    public static String loadHtml(String url) {
        return loadHtml(url, "", false);
    }

    public static String loadLine(String url, int line) {
        return "@ll" + url + RS + line;
    }

    public static String loadIni(String url, String name, boolean isINILike) {
        return "@li" + url + RS + name + (isINILike ? RS + "1" : "");
    }

    public static String loadIni(String url, String name) {
        return loadIni(url, name, false);
    }

    // Name: Name Or Nested Paths. Is Supprt Index (Student[8].Name). Nested Paths Index Starts At 0
    public static String loadJson(String url, String name) {
        return "@lj" + url + RS + name;
    }

    // Name: Name Or XPath; XPath Index Starts At 1
    public static String loadXml(String url, String name) {
        return "@lx" + url + RS + name;
    }

    // MethodName: It's Check Function Or Variable
    public static String hasMethod(String methodName) {
        return "@hm" + methodName;
    }

    public static String hasModuleMethod(String methodName) {
        return "@hM" + methodName;
    }

    // This Method Return True Or False If Key Pressed
    // Modifier: Alt, AltGraph, Control, Meta, Shift, CapsLock, NumLock, ScrollLock
    public static String getModifierState(String modifier) {
        return "@ms" + modifier;
    }

    // Math
    public static String math(String methodName, Object[] args) {
        String returnValue = "@M#" + methodName;
        if (args != null) {
            returnValue += (args.length > 0) ? RS + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        return returnValue;
    }

    public static String math(String methodName) {
        return math(methodName, null);
    }

    // Date
    public static final String DATE_YEAR = "@dy";
    // Month In JavaScript Is Start From Index 0, Month In WebForms Core Is Start From Index 1 
    public static final String DATE_MONTH = "@dm";
    public static final String DATE_DAY = "@dd";
    public static final String DATE_DATE = "@dD";
    public static final String DATE_HOURS = "@dh";
    public static final String DATE_MINUTES = "@di";
    public static final String DATE_SECONDS = "@ds";
    public static final String DATE_MILLISECONDS = "@dl";

    // String
    public static final String SPACE = "@sp";
    public static final String AT_SIGN = "@sa";

    // Tag
    public static String getId(String inputPlace) {
        return "@$i" + inputPlace;
    }

    public static String getName(String inputPlace) {
        return "@$n" + inputPlace;
    }

    public static String getValue(String inputPlace) {
        return "@$v" + inputPlace;
    }

    public static String getValueLength(String inputPlace) {
        return "@$e" + inputPlace;
    }

    public static String getClass(String inputPlace) {
        return "@$c" + inputPlace;
    }

    public static String getStyle(String inputPlace) {
        return "@$s" + inputPlace;
    }

    public static String getTitle(String inputPlace) {
        return "@$l" + inputPlace;
    }

    public static String getLabel(String inputPlace) {
        return "@$A" + inputPlace;
    }

    public static String getText(String inputPlace) {
        return "@$t" + inputPlace;
    }

    public static String getOuterText(String inputPlace) {
        return "@$o" + inputPlace;
    }

    public static String getTextLength(String inputPlace) {
        return "@$g" + inputPlace;
    }

    public static String getAttribute(String inputPlace, String attribute) {
        return "@$a" + inputPlace + RS + attribute;
    }

    public static String getWidth(String inputPlace) {
        return "@$w" + inputPlace;
    }

    public static String getHeight(String inputPlace) {
        return "@$h" + inputPlace;
    }

    public static String getIsReadOnly(String inputPlace) {
        return "@$r" + inputPlace;
    }

    public static String getSelectedIndex(String inputPlace) {
        return "@$x" + inputPlace;
    }

    public static String getIndex(String inputPlace) {
        return "@$I" + inputPlace;
    }

    public static String getTextAlign(String inputPlace) {
        return "@$T" + inputPlace;
    }

    public static String getNodeLength(String inputPlace) {
        return "@$L" + inputPlace;
    }

    public static String getIsVisible(String inputPlace) {
        return "@$V" + inputPlace;
    }
	
	public static String getTagHash(String inputPlace) {
		return "@$H" + inputPlace;
	}

    // Save
    public static String hasHash(String hash) {
        return "@HH" + hash;
    }

    public static String cookie(String key) {
        return "@co" + key;
    }

    public static String save(String key, String replaceValue) {
        return "@cs" + key + RS + replaceValue;
    }

    public static String save(String key) {
        return "@cs" + key;
    }

    public static String save() {
        return save(".");
    }

    public static String saveThenRemove(String key) {
        return "@cl" + key;
    }

    public static String saveLength(String key) {
        return "@cg" + key;
    }

    public static String cache(String key, String replaceValue) {
        return "@cd" + key + RS + replaceValue;
    }

    public static String cache(String key) {
        return "@cd" + key;
    }

    public static String cache() {
        return cache(".");
    }

    public static String cacheThenRemove(String key) {
        return "@ct" + key;
    }

    public static String cacheLength(String key) {
        return "@cG" + key;
    }

    public static String saveLine(String key, int line) {
        return "@lL" + key + "[" + line;
    }

    public static String saveLine(String key) {
        return saveLine(key, 0);
    }

    public static String saveLine() {
        return saveLine(".", 0);
    }

    public static String saveLineConsume(String key) {
        return "@lL" + key;
    }

    public static String saveLineConsume() {
        return saveLineConsume(".");
    }

    // INIKey: Only Direct Key is Supported
    public static String saveIni(String key, String iniKey) {
        return "@lI" + key + "[" + iniKey;
    }

    public static String cacheLine(String key, int line) {
        return "@dL" + key + "[" + line;
    }

    public static String cacheLine(String key) {
        return cacheLine(key, 0);
    }

    public static String cacheLine() {
        return cacheLine(".", 0);
    }

    public static String cacheLineConsume(String key) {
        return "@dL" + key;
    }

    public static String cacheLineConsume() {
        return cacheLineConsume(".");
    }

    // INIKey: Only Direct Key is Supported
    public static String cacheIni(String key, String iniKey) {
        return "@dI" + key + "[" + iniKey;
    }

    // Format Storage
    public static String formatStore(String key) {
        return "@fr" + key;
    }

    public static String formatStoreByXmlQuery(String key, String xpath) {
        return "@fx" + key + RS + xpath;
    }

    public static String formatStoreByJsonQuery(String key, String query) {
        return "@fj" + key + RS + query;
    }

    public static String formatStoreByIni(String key, String name) {
        return "@fi" + key + RS + name;
    }

    public static String formatStoreByText(String key, int line) {
        return "@ft" + key + RS + line;
    }

    public static String formatStoreByVariable(String key) {
        return "@fv" + key;
    }

    // State
    public static String hasState(String path) {
        return "@hs" + path;
    }

    // SSE
    public static String sseIsConnected(String path) {
        return "@Sc" + path;
    }

    // WebSockets
    public static String webSocketsIsConnected(String path) {
        return "@Wc" + path;
    }

    public static String webSocketsIsConnected() {
        return webSocketsIsConnected("");
    }

    // Document
    public static final String TAB_IS_ACTIVE = "@da";

    // Window
    public static final String HREF = "@wf";
    public static final String PATH_NAME = "@wP";

    public static String query(String name) {
        return "@wq" + name;
    }

    public static String query() {
        return query("*");
    }

    public static final String HASH = "@wh";
    public static final String HOST = "@wH";
    public static final String HOST_NAME = "@wn";
    public static final String PORT = "@wT";
    public static final String ORIGIN = "@wo";
    public static final String GET_SELECTION = "@ws";
    public static final String SCROLL_X = "@wx";
    public static final String SCROLL_Y = "@wy";

    public static String segment(int index) {
        return "@wS" + index;
    }

    // It Only Works when the String Starts with the Tilde Character (~). The Path is Also Separated by the Slash Character (/). #~/Segment1/Segment2/Segment3
    public static String hashSegment(int index) {
        return "@wt" + index;
    }

    // Navigator
    public static final String CLIPBOARD_TEXT = "@nC";
    public static final String GEO_LATITUDE = "@nW";
    public static final String GEO_LONGITUDE = "@nO";
    public static final String LANGUAGE = "@nL";
    public static final String IS_ON_LINE = "@no";
    public static final String USER_AGENT = "@na";

    // Screen
    public static final String SCREEN_WIDTH = "@sw";
    public static final String SCREEN_HEIGHT = "@sh";
    public static final String SCREEN_ORIENTATION_TYPE = "@so";
    public static final String SCREEN_ORIENTATION_ANGLE = "@sr";

    // Performance
    public static final String TIME_ORIGIN = "@pt";
    public static final String PERFORMANCE_NOW = "@pn";

    // Event
    public static final String EVENT = "@EV";
    public static final String EVENT_SERIALIZE = "@Es";
    public static final String EVENT_KEY = "@ek";
    public static final String EVENT_WHICH = "@ew";
    public static final String EVENT_CLIENT_X = "@ex";
    public static final String EVENT_CLIENT_Y = "@ey";
    public static final String EVENT_PAGE_X = "@eX";
    public static final String EVENT_PAGE_Y = "@eY";
    public static final String EVENT_OFFSET_X = "@Ex";
    public static final String EVENT_OFFSET_Y = "@Ey";
    public static final String EVENT_DELTA_Y = "@ed";

    private static String[] toStringArray(Object[] args) {
        String[] result = new String[args.length];
        for (int i = 0; i < args.length; i++) {
            result[i] = args[i] != null ? args[i].toString() : "";
        }
        return result;
    }
}
