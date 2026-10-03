// WebForms.java 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

package webformscore;

public class WebForms {
    private static final char GS = (char) 29;
    private static final char US = (char) 31;

    private StringBuilder webFormsData = new StringBuilder();

    void add(String name, String value) {
        if (webFormsData.length() > 0) {
            webFormsData.append('\n');
        }
        webFormsData.append(name);
        webFormsData.append('=');
        webFormsData.append(value);
    }

    void add(String name) {
        if (webFormsData.length() > 0) {
            webFormsData.append('\n');
        }
        webFormsData.append(name);
    }

    void addToUp(String name, String value) {
        String line = name + "=" + value;
        if (webFormsData.length() > 0) {
            line += "\n";
        }
        webFormsData.insert(0, line);
    }

    void addToUp(String name) {
        String line = name;
        if (webFormsData.length() > 0) {
            line += "\n";
        }
        webFormsData.insert(0, line);
    }

    String getLineByIndex(int index) {
        if (webFormsData.length() == 0) {
            return "";
        }
        String data = webFormsData.toString();
        String[] lines = data.split("\n", -1);
        if (index < 0) {
            index = lines.length + index;
        }
        if (index < 0 || index >= lines.length) {
            return "";
        }
        return lines[index];
    }

    void updateLineByIndex(int index, String name, String value) {
        if (webFormsData.length() == 0) {
            return;
        }
        String data = webFormsData.toString();
        String[] lines = data.split("\n", -1);
        if (index < 0) {
            index = lines.length + index;
        }
        if (index < 0 || index >= lines.length) {
            return;
        }
        lines[index] = name + ((value == null || value.isEmpty()) ? "" : "=" + value);
        webFormsData.setLength(0);
        webFormsData.append(String.join("\n", lines));
    }

    // For Extension
    public void addLine(String name, String value) {
        add(name, value);
    }

    // Add
    // Creates the Data if it does not exist; otherwise, Appends the New Value to the Existing Value.
    public void addId(String inputPlace, String id) {
        add("ai" + inputPlace, id);
    }

    public void addName(String inputPlace, String name) {
        add("an" + inputPlace, name);
    }

    public void addValue(String inputPlace, String value) {
        add("av" + inputPlace, value);
    }

    public void addClass(String inputPlace, String className) {
        add("ac" + inputPlace, className);
    }

    public void addStyle(String inputPlace, String style) {
        add("as" + inputPlace, style);
    }

    public void addStyle(String inputPlace, String name, String value) {
        add("as" + inputPlace, name + ':' + value);
    }

    public void addOptionTag(String inputPlace, String text, String value, boolean selected) {
        add("ao" + inputPlace, value + GS + text + (selected ? GS + "1" : ""));
    }

    public void addOptionTag(String inputPlace, String text, String value) {
        addOptionTag(inputPlace, text, value, false);
    }

    public void addCheckBoxTag(String inputPlace, String text, String value, boolean checked) {
        add("ak" + inputPlace, value + GS + text + (checked ? GS + "1" : ""));
    }

    public void addCheckBoxTag(String inputPlace, String text, String value) {
        addCheckBoxTag(inputPlace, text, value, false);
    }

    public void addTitle(String inputPlace, String title) {
        add("al" + inputPlace, title);
    }

    public void addLabel(String inputPlace, String label) {
        add("aA" + inputPlace, label);
    }

    public void addText(String inputPlace, String text) {
        add("at" + inputPlace, text.replace("\n", "$[ln];"));
    }

    public void addTextToUp(String inputPlace, String text) {
        add("pt" + inputPlace, text.replace("\n", "$[ln];"));
    }

    public void addAttribute(String inputPlace, String attribute, String value, char splitter) {
        add("aa" + inputPlace, attribute + GS + (splitter != '\0' ? String.valueOf(splitter) : "") + ((value != null && !value.isEmpty()) ? GS + value : ""));
    }

    public void addAttribute(String inputPlace, String attribute, String value) {
        addAttribute(inputPlace, attribute, value, '\0');
    }

    public void addAttribute(String inputPlace, String attribute) {
        addAttribute(inputPlace, attribute, "", '\0');
    }

    public void addTag(String inputPlace, String tagName, String id) {
        add("nt" + inputPlace, tagName + ((id != null && !id.isEmpty()) ? GS + id : ""));
    }

    public void addTag(String inputPlace, String tagName) {
        addTag(inputPlace, tagName, "");
    }

    public void addTagToUp(String inputPlace, String tagName, String id) {
        add("ut" + inputPlace, tagName + ((id != null && !id.isEmpty()) ? GS + id : ""));
    }

    public void addTagToUp(String inputPlace, String tagName) {
        addTagToUp(inputPlace, tagName, "");
    }

    public void addTagBefore(String inputPlace, String tagName, String id) {
        add("bt" + inputPlace, tagName + ((id != null && !id.isEmpty()) ? GS + id : ""));
    }

    public void addTagBefore(String inputPlace, String tagName) {
        addTagBefore(inputPlace, tagName, "");
    }

    public void addTagAfter(String inputPlace, String tagName, String id) {
        add("ft" + inputPlace, tagName + ((id != null && !id.isEmpty()) ? GS + id : ""));
    }

    public void addTagAfter(String inputPlace, String tagName) {
        addTagAfter(inputPlace, tagName, "");
    }

    public void addHidden(String inputPlace, String name, String value, String id) {
        add("ah" + inputPlace, name + GS + value + ((id != null && !id.isEmpty()) ? GS + id : ""));
    }

    public void addHidden(String inputPlace, String name, String value) {
        addHidden(inputPlace, name, value, "");
    }

    // Set
    // Creates the Data if it does not exist; otherwise, Replaces the Existing Value with the New Value.
    public void setId(String inputPlace, String id) {
        add("si" + inputPlace, id);
    }

    public void setName(String inputPlace, String name) {
        add("sn" + inputPlace, name);
    }

    public void setValue(String inputPlace, String value) {
        add("sv" + inputPlace, value);
    }

    public void setClass(String inputPlace, String className) {
        add("sc" + inputPlace, className);
    }

    public void setStyle(String inputPlace, String style) {
        add("ss" + inputPlace, style);
    }

    public void setStyle(String inputPlace, String name, String value) {
        add("ss" + inputPlace, name + ':' + value);
    }

    public void setOptionTag(String inputPlace, String text, String value, boolean selected) {
        add("so" + inputPlace, value + GS + text + (selected ? GS + "1" : ""));
    }

    public void setOptionTag(String inputPlace, String text, String value) {
        setOptionTag(inputPlace, text, value, false);
    }

    public void setChecked(String inputPlace, boolean checked) {
        add("sk" + inputPlace, checked ? "1" : "0");
    }

    public void setChecked(String inputPlace) {
        setChecked(inputPlace, false);
    }

    public void setCheckBoxTag(String inputPlace, String text, String value, boolean checked) {
        add("sk" + inputPlace, value + GS + text + (checked ? GS + "1" : ""));
    }

    public void setCheckBoxTag(String inputPlace, String text, String value) {
        setCheckBoxTag(inputPlace, text, value, false);
    }

    public void setTitle(String inputPlace, String title) {
        add("sl" + inputPlace, title);
    }

    public void setLabel(String inputPlace, String label) {
        add("sA" + inputPlace, label);
    }

    public void setText(String inputPlace, String text) {
        add("st" + inputPlace, text.replace("\n", "$[ln];"));
    }

    public void setAttribute(String inputPlace, String attribute, String value) {
        add("sa" + inputPlace, attribute + GS + ((value != null && !value.isEmpty()) ? GS + value : ""));
    }

    public void setAttribute(String inputPlace, String attribute) {
        setAttribute(inputPlace, attribute, "");
    }

    public void setWidth(String inputPlace, String width) {
        add("sw" + inputPlace, width);
    }

    public void setWidth(String inputPlace, int width) {
        setWidth(inputPlace, String.valueOf(width) + "px");
    }

    public void setHeight(String inputPlace, String height) {
        add("sh" + inputPlace, height);
    }

    public void setHeight(String inputPlace, int height) {
        setHeight(inputPlace, String.valueOf(height) + "px");
    }

    public void setBackgroundColor(String inputPlace, String color) {
        add("bc" + inputPlace, color);
    }

    public void setTextColor(String inputPlace, String color) {
        add("tc" + inputPlace, color);
    }

    public void setFontName(String inputPlace, String name) {
        add("fn" + inputPlace, name);
    }

    public void setFontSize(String inputPlace, String size) {
        add("fs" + inputPlace, size);
    }

    public void setFontSize(String inputPlace, int size) {
        add("fs" + inputPlace, String.valueOf(size) + "px");
    }

    public void setFontBold(String inputPlace, boolean bold) {
        add("fb" + inputPlace, bold ? "1" : "0");
    }

    public void setVisible(String inputPlace, boolean visible) {
        add("vi" + inputPlace, visible ? "1" : "0");
    }

    public void setTextAlign(String inputPlace, String align) {
        add("ta" + inputPlace, align);
    }

    public void setReadOnly(String inputPlace, boolean readOnly) {
        add("sr" + inputPlace, readOnly ? "1" : "0");
    }

    public void setDisabled(String inputPlace, boolean disabled) {
        add("sd" + inputPlace, disabled ? "1" : "0");
    }

    public void setFocus(String inputPlace, boolean focus) {
        add("sf" + inputPlace, focus ? "1" : "0");
    }

    public void setMinLength(String inputPlace, String length) {
        add("mn" + inputPlace, length);
    }

    public void setMinLength(String inputPlace, int length) {
        setMinLength(inputPlace, String.valueOf(length));
    }

    public void setMaxLength(String inputPlace, String length) {
        add("mx" + inputPlace, length);
    }

    public void setMaxLength(String inputPlace, int length) {
        setMaxLength(inputPlace, String.valueOf(length));
    }

    public void setSelectedValue(String inputPlace, String value) {
        add("ts" + inputPlace, value);
    }

    public void setSelectedIndex(String inputPlace, String index) {
        add("ti" + inputPlace, index);
    }

    public void setSelectedIndex(String inputPlace, int index) {
        setSelectedIndex(inputPlace, String.valueOf(index));
    }

    public void setCheckedValue(String inputPlace, String value, boolean checked) {
        add("ks" + inputPlace, value + GS + (checked ? "1" : "0"));
    }

    public void setCheckedIndex(String inputPlace, String index, boolean checked) {
        add("ki" + inputPlace, index + GS + (checked ? "1" : "0"));
    }

    public void setCheckedIndex(String inputPlace, int index, boolean checked) {
        setCheckedIndex(inputPlace, String.valueOf(index), checked);
    }
	
	public void setCustomValidity(String inputPlace, String text) {
		add("cv" + inputPlace, text.replace("\n", "$[ln];"));
	}

    // Insert
    // Creates the Data only if it does not exist; otherwise, does nothing.
    public void insertId(String inputPlace, String id) {
        add("ii" + inputPlace, id);
    }

    public void insertName(String inputPlace, String name) {
        add("in" + inputPlace, name);
    }

    public void insertValue(String inputPlace, String value) {
        add("iv" + inputPlace, value);
    }

    public void insertClass(String inputPlace, String className) {
        add("ic" + inputPlace, className);
    }

    public void insertStyle(String inputPlace, String style) {
        add("is" + inputPlace, style);
    }

    public void insertStyle(String inputPlace, String name, String value) {
        add("is" + inputPlace, name + ':' + value);
    }

    public void insertOptionTag(String inputPlace, String text, String value, boolean selected) {
        add("io" + inputPlace, value + GS + text + (selected ? GS + "1" : ""));
    }

    public void insertOptionTag(String inputPlace, String text, String value) {
        insertOptionTag(inputPlace, text, value, false);
    }

    public void insertCheckBoxTag(String inputPlace, String text, String value, boolean checked) {
        add("ik" + inputPlace, value + GS + text + (checked ? GS + "1" : ""));
    }

    public void insertCheckBoxTag(String inputPlace, String text, String value) {
        insertCheckBoxTag(inputPlace, text, value, false);
    }

    public void insertTitle(String inputPlace, String title) {
        add("il" + inputPlace, title);
    }

    public void insertLabel(String inputPlace, String label) {
        add("iA" + inputPlace, label);
    }

    public void insertText(String inputPlace, String text) {
        add("it" + inputPlace, text.replace("\n", "$[ln];"));
    }

    public void insertAttribute(String inputPlace, String attribute, String value, char splitter) {
        add("ia" + inputPlace, attribute + GS + (splitter != '\0' ? String.valueOf(splitter) : "") + ((value != null && !value.isEmpty()) ? GS + value : ""));
    }

    public void insertAttribute(String inputPlace, String attribute, String value) {
        insertAttribute(inputPlace, attribute, value, '\0');
    }

    public void insertAttribute(String inputPlace, String attribute) {
        insertAttribute(inputPlace, attribute, "", '\0');
    }

    // Delete
    public void deleteId(String inputPlace) {
        add("di" + inputPlace);
    }

    public void deleteName(String inputPlace) {
        add("dn" + inputPlace);
    }

    public void deleteValue(String inputPlace) {
        add("dv" + inputPlace);
    }

    public void deleteClass(String inputPlace, String className) {
        add("dc" + inputPlace, className);
    }

    public void deleteStyle(String inputPlace, String styleName) {
        add("ds" + inputPlace, styleName);
    }

    public void deleteOptionTag(String inputPlace, String value) {
        add("do" + inputPlace, value);
    }

    public void deleteAllOptionTag(String inputPlace) {
        add("do" + inputPlace, "*");
    }

    public void deleteCheckBoxTag(String inputPlace, String value) {
        add("dk" + inputPlace, value);
    }

    public void deleteAllCheckBoxTag(String inputPlace) {
        add("dk" + inputPlace, "*");
    }

    public void deleteTitle(String inputPlace) {
        add("dl" + inputPlace);
    }

    public void deleteLabel(String inputPlace) {
        add("dA" + inputPlace);
    }

    public void deleteText(String inputPlace) {
        add("dt" + inputPlace);
    }

    public void deleteAttribute(String inputPlace, String attribute) {
        add("da" + inputPlace, attribute);
    }

    public void delete(String inputPlace) {
        add("de" + inputPlace);
    }

    public void deleteParent(String inputPlace) {
        add("dp" + inputPlace);
    }

    // Tag Transformation
    public void swapTag(String inputPlace, String outputPlace) {
        add("sp" + inputPlace, outputPlace);
    }

    public void setReflect(String inputPlace, String tag) {
        add("sR" + inputPlace, tag);
    }

    public void setReflectByOutputPlace(String inputPlace, String outputPlace) {
        add("iR" + inputPlace, outputPlace);
    }

    public void setMorph(String inputPlace, String tag) {
        add("sM" + inputPlace, tag);
    }

    public void setMorphByOutputPlace(String inputPlace, String outputPlace) {
        add("iM" + inputPlace, outputPlace);
    }
	
	// The Snapshot and Rollback Mechanism is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
	// inputPlace: Only One Tag
	public void snapshot(String inputPlace, String key, boolean permanent) {
		add("rS" + inputPlace, ((key == null || key.isEmpty()) ? inputPlace : key) + (permanent ? GS + "1" : ""));
	}

	public void snapshot(String inputPlace) {
		snapshot(inputPlace, "", false);
	}

	public void snapshot(String inputPlace, String key) {
		snapshot(inputPlace, key, false);
	}
	
	public void rollback(String inputPlace, String key, boolean permanent) {
		add("rB" + inputPlace, ((key == null || key.isEmpty()) ? inputPlace : key) + (permanent ? GS + "1" : ""));
	}

	public void rollback(String inputPlace) {
		rollback(inputPlace, "", false);
	}

	public void rollback(String inputPlace, String key) {
		rollback(inputPlace, key, false);
	}

    // Browser
    public void changeUrl(String url) {
        add("cu", url);
    }

    public void setHeadTitle(String title) {
        add("ht", title);
    }

    public void clipboardWriteText(String text) {
        add("nw", text);
    }

    public void scrollTo(String x, String y) {
        add("ws", x + GS + y);
    }

    public void scrollTo(int x, int y) {
        scrollTo(String.valueOf(x), String.valueOf(y));
    }

    public void historyGo(String steps) {
        add("wg", steps);
    }

    public void historyGo(int steps) {
        historyGo(String.valueOf(steps));
    }

    public void reloadPage() {
        add("lr");
    }

    public void redirect(String path) {
        add("lh", path);
    }

    // Increase
    public void increaseMinLength(String inputPlace, String value) {
        add("+n" + inputPlace, value);
    }

    public void increaseMinLength(String inputPlace, int value) {
        increaseMinLength(inputPlace, String.valueOf(value));
    }

    public void increaseMaxLength(String inputPlace, String value) {
        add("+x" + inputPlace, value);
    }

    public void increaseMaxLength(String inputPlace, int value) {
        increaseMaxLength(inputPlace, String.valueOf(value));
    }

    public void increaseFontSize(String inputPlace, String value) {
        add("+f" + inputPlace, value);
    }

    public void increaseFontSize(String inputPlace, int value) {
        increaseFontSize(inputPlace, String.valueOf(value));
    }

    public void increaseWidth(String inputPlace, String value) {
        add("+w" + inputPlace, value);
    }

    public void increaseWidth(String inputPlace, int value) {
        increaseWidth(inputPlace, String.valueOf(value));
    }

    public void increaseHeight(String inputPlace, String value) {
        add("+h" + inputPlace, value);
    }

    public void increaseHeight(String inputPlace, int value) {
        increaseHeight(inputPlace, String.valueOf(value));
    }

    public void increaseValue(String inputPlace, String value) {
        add("+v" + inputPlace, value);
    }

    public void increaseValue(String inputPlace, int value) {
        increaseValue(inputPlace, String.valueOf(value));
    }

    // Decrease
    public void decreaseMinLength(String inputPlace, String value) {
        add("-n" + inputPlace, value);
    }

    public void decreaseMinLength(String inputPlace, int value) {
        decreaseMinLength(inputPlace, String.valueOf(value));
    }

    public void decreaseMaxLength(String inputPlace, String value) {
        add("-x" + inputPlace, value);
    }

    public void decreaseMaxLength(String inputPlace, int value) {
        decreaseMaxLength(inputPlace, String.valueOf(value));
    }

    public void decreaseFontSize(String inputPlace, String value) {
        add("-f" + inputPlace, value);
    }

    public void decreaseFontSize(String inputPlace, int value) {
        decreaseFontSize(inputPlace, String.valueOf(value));
    }

    public void decreaseWidth(String inputPlace, String value) {
        add("-w" + inputPlace, value);
    }

    public void decreaseWidth(String inputPlace, int value) {
        decreaseWidth(inputPlace, String.valueOf(value));
    }

    public void decreaseHeight(String inputPlace, String value) {
        add("-h" + inputPlace, value);
    }

    public void decreaseHeight(String inputPlace, int value) {
        decreaseHeight(inputPlace, String.valueOf(value));
    }

    public void decreaseValue(String inputPlace, String value) {
        add("-v" + inputPlace, value);
    }

    public void decreaseValue(String inputPlace, int value) {
        decreaseValue(inputPlace, String.valueOf(value));
    }

    // Event
    // ConstructorName: mouseevent, keyboardevent, uievent, focusevent, inputevent, event
    // All Method in "Event" Section Only Support Dynamic Args Once. To Support Invoking Dynamic Arguments on a Momentary Basis, Use "EventListener" Section Methods.
    public void triggerEvent(String inputPlace, String htmlEventListener, String constructorName) {
        add("TE" + inputPlace, htmlEventListener + ((constructorName != null && !constructorName.isEmpty()) ? GS + constructorName : ""));
    }

    public void triggerEvent(String inputPlace, String htmlEventListener) {
        triggerEvent(inputPlace, htmlEventListener, null);
    }

    public void setPostEvent(String inputPlace, String htmlEvent) {
        add("Ep" + inputPlace, htmlEvent);
    }

    public void setPostEvent(String inputPlace, String htmlEvent, String outputPlace) {
        add("Ep" + inputPlace, htmlEvent + GS + outputPlace);
    }

    public void setPostEventAddView(String inputPlace, String htmlEvent) {
        add("Ep" + inputPlace, htmlEvent + GS + "+");
    }

    public void setPostEventListener(String inputPlace, String htmlEventListener) {
        add("EP" + inputPlace, htmlEventListener);
    }

    public void setPostEventListener(String inputPlace, String htmlEventListener, String outputPlace) {
        add("EP" + inputPlace, htmlEventListener + GS + outputPlace);
    }

    public void setPostEventListenerAddView(String inputPlace, String htmlEventListener) {
        add("EP" + inputPlace, htmlEventListener + GS + "+");
    }

    public void setGetEvent(String inputPlace, String htmlEvent, String path) {
        add("Eg" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setGetEvent(String inputPlace, String htmlEvent) {
        setGetEvent(inputPlace, htmlEvent, null);
    }

    public void setGetEvent(String inputPlace, String htmlEvent, String outputPlace, String path) {
        add("Eg" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setGetEventListener(String inputPlace, String htmlEventListener, String path) {
        add("EG" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setGetEventListener(String inputPlace, String htmlEventListener) {
        setGetEventListener(inputPlace, htmlEventListener, null);
    }

    public void setGetEventListener(String inputPlace, String htmlEventListener, String outputPlace, String path) {
        add("EG" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setPutEvent(String inputPlace, String htmlEvent, String path) {
        add("Et" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setPutEvent(String inputPlace, String htmlEvent) {
        setPutEvent(inputPlace, htmlEvent, null);
    }

    public void setPutEvent(String inputPlace, String htmlEvent, String outputPlace, String path) {
        add("Et" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setPutEventListener(String inputPlace, String htmlEventListener, String path) {
        add("ET" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setPutEventListener(String inputPlace, String htmlEventListener) {
        setPutEventListener(inputPlace, htmlEventListener, null);
    }

    public void setPutEventListener(String inputPlace, String htmlEventListener, String outputPlace, String path) {
        add("ET" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setPatchEvent(String inputPlace, String htmlEvent, String path) {
        add("Ea" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setPatchEvent(String inputPlace, String htmlEvent) {
        setPatchEvent(inputPlace, htmlEvent, null);
    }

    public void setPatchEvent(String inputPlace, String htmlEvent, String outputPlace, String path) {
        add("Ea" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setPatchEventListener(String inputPlace, String htmlEventListener, String path) {
        add("EA" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setPatchEventListener(String inputPlace, String htmlEventListener) {
        setPatchEventListener(inputPlace, htmlEventListener, null);
    }

    public void setPatchEventListener(String inputPlace, String htmlEventListener, String outputPlace, String path) {
        add("EA" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setDeleteEvent(String inputPlace, String htmlEvent, String path) {
        add("El" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setDeleteEvent(String inputPlace, String htmlEvent) {
        setDeleteEvent(inputPlace, htmlEvent, null);
    }

    public void setDeleteEvent(String inputPlace, String htmlEvent, String outputPlace, String path) {
        add("El" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setDeleteEventListener(String inputPlace, String htmlEventListener, String path) {
        add("EL" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setDeleteEventListener(String inputPlace, String htmlEventListener) {
        setDeleteEventListener(inputPlace, htmlEventListener, null);
    }

    public void setDeleteEventListener(String inputPlace, String htmlEventListener, String outputPlace, String path) {
        add("EL" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setOptionsEvent(String inputPlace, String htmlEvent, String path) {
        add("Eo" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setOptionsEvent(String inputPlace, String htmlEvent) {
        setOptionsEvent(inputPlace, htmlEvent, null);
    }

    public void setOptionsEvent(String inputPlace, String htmlEvent, String outputPlace, String path) {
        add("Eo" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setOptionsEventListener(String inputPlace, String htmlEventListener, String path) {
        add("EO" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setOptionsEventListener(String inputPlace, String htmlEventListener) {
        setOptionsEventListener(inputPlace, htmlEventListener, null);
    }

    public void setOptionsEventListener(String inputPlace, String htmlEventListener, String outputPlace, String path) {
        add("EO" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + outputPlace);
    }

    public void setHeadEvent(String inputPlace, String htmlEvent, String path) {
        add("Eh" + inputPlace, htmlEvent + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setHeadEvent(String inputPlace, String htmlEvent) {
        setHeadEvent(inputPlace, htmlEvent, null);
    }

    public void setHeadEventListener(String inputPlace, String htmlEventListener, String path) {
        add("EH" + inputPlace, htmlEventListener + GS + ((path != null && !path.isEmpty()) ? path : "#"));
    }

    public void setHeadEventListener(String inputPlace, String htmlEventListener) {
        setHeadEventListener(inputPlace, htmlEventListener, null);
    }

    // IsMultiPart: If this value is true, the data will be sent based on the Form and with the "content" key.
    public void setSendEvent(String inputPlace, String htmlEvent, String data, String path, String method, boolean isMultiPart, String contentType, String outputPlace) {
        add("En" + inputPlace, htmlEvent + GS + data.replace("\n", "$[ln];").replace("\"", "$[dq];").replace("'", "$[sq];") + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + method + GS + (isMultiPart ? "1" : "0") + GS + contentType + GS + outputPlace);
    }

    public void setSendEvent(String inputPlace, String htmlEvent, String data) {
        setSendEvent(inputPlace, htmlEvent, data, null, "POST", false, "text/plain", null);
    }

    public void setSendEventListener(String inputPlace, String htmlEventListener, String data, String path, String method, boolean isMultiPart, String contentType, String outputPlace) {
        add("EN" + inputPlace, htmlEventListener + GS + data.replace("\n", "$[ln];") + GS + ((path != null && !path.isEmpty()) ? path : "#") + GS + method + GS + (isMultiPart ? "1" : "0") + GS + contentType + GS + outputPlace);
    }

    public void setSendEventListener(String inputPlace, String htmlEventListener, String data) {
        setSendEventListener(inputPlace, htmlEventListener, data, null, "POST", false, "text/plain", null);
    }

    public void setCommentEvent(String inputPlace, String htmlEvent, String index, String outputPlace) {
        add("Eb" + inputPlace, htmlEvent + GS + index + GS + outputPlace);
    }

    public void setCommentEvent(String inputPlace, String htmlEvent) {
        setCommentEvent(inputPlace, htmlEvent, null, null);
    }

    public void setCommentEvent(String inputPlace, String htmlEvent, int index, String outputPlace) {
        setCommentEvent(inputPlace, htmlEvent, String.valueOf(index), outputPlace);
    }

    public void setCommentEventListener(String inputPlace, String htmlEventListener, String index, String outputPlace) {
        add("EB" + inputPlace, htmlEventListener + GS + index + GS + outputPlace);
    }

    public void setCommentEventListener(String inputPlace, String htmlEventListener) {
        setCommentEventListener(inputPlace, htmlEventListener, null, null);
    }

    public void setCommentEventListener(String inputPlace, String htmlEventListener, int index, String outputPlace) {
        setCommentEventListener(inputPlace, htmlEventListener, String.valueOf(index), outputPlace);
    }

    public void setWasmEvent(String inputPlace, String htmlEvent, String wasmLanguage, String wasmUrl, String methodName, Object[] args, String outputPlace) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("Ey" + inputPlace, htmlEvent + GS + wasmLanguage + GS + wasmUrl + GS + methodName + GS + argsJoin + GS + outputPlace);
    }

    public void setWasmEvent(String inputPlace, String htmlEvent, String wasmLanguage, String wasmUrl, String methodName) {
        setWasmEvent(inputPlace, htmlEvent, wasmLanguage, wasmUrl, methodName, null, null);
    }

    public void setWasmEventListener(String inputPlace, String htmlEventListener, String wasmLanguage, String wasmUrl, String methodName, Object[] args, String outputPlace) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("EY" + inputPlace, htmlEventListener + GS + wasmLanguage + GS + wasmUrl + GS + methodName + GS + argsJoin + GS + outputPlace);
    }

    public void setWasmEventListener(String inputPlace, String htmlEventListener, String wasmLanguage, String wasmUrl, String methodName) {
        setWasmEventListener(inputPlace, htmlEventListener, wasmLanguage, wasmUrl, methodName, null, null);
    }

    public void setWebSocketEvent(String inputPlace, String htmlEvent, String path) {
        add("Ew" + inputPlace, htmlEvent + GS + path);
    }

    public void setWebSocketEventListener(String inputPlace, String htmlEventListener, String path) {
        add("EW" + inputPlace, htmlEventListener + GS + path);
    }

    public void setSSEEvent(String inputPlace, String htmlEvent, String path, boolean shouldReconnect, int reconnectTryTimeout) {
        add("Ee" + inputPlace, htmlEvent + GS + path + GS + (shouldReconnect ? "1" : "0") + GS + String.valueOf(reconnectTryTimeout));
    }

    public void setSSEEvent(String inputPlace, String htmlEvent, String path) {
        setSSEEvent(inputPlace, htmlEvent, path, true, 3000);
    }

    public void setSSEEvent(String inputPlace, String htmlEvent, String path, String outputPlace, boolean shouldReconnect, int reconnectTryTimeout) {
        add("Ee" + inputPlace, htmlEvent + GS + path + GS + (shouldReconnect ? "1" : "0") + GS + String.valueOf(reconnectTryTimeout) + GS + outputPlace);
    }

    public void setSSEEventListener(String inputPlace, String htmlEventListener, String path, boolean shouldReconnect, int reconnectTryTimeout) {
        add("EE" + inputPlace, htmlEventListener + GS + path + GS + (shouldReconnect ? "1" : "0") + GS + String.valueOf(reconnectTryTimeout));
    }

    public void setSSEEventListener(String inputPlace, String htmlEventListener, String path) {
        setSSEEventListener(inputPlace, htmlEventListener, path, true, 3000);
    }

    public void setSSEEventListener(String inputPlace, String htmlEventListener, String path, String outputPlace, boolean shouldReconnect, int reconnectTryTimeout) {
        add("EE" + inputPlace, htmlEventListener + GS + path + GS + (shouldReconnect ? "1" : "0") + GS + String.valueOf(reconnectTryTimeout) + GS + outputPlace);
    }

    public void setFrontEvent(String inputPlace, String htmlEvent, String modulePath, Object[] args, String outputPlace) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("Ej" + inputPlace, htmlEvent + GS + modulePath + GS + outputPlace + argsJoin);
    }

    public void setFrontEvent(String inputPlace, String htmlEvent, String modulePath) {
        setFrontEvent(inputPlace, htmlEvent, modulePath, null, null);
    }

    public void setFrontEventListener(String inputPlace, String htmlEventListener, String modulePath, Object[] args, String outputPlace) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("EJ" + inputPlace, htmlEventListener + GS + modulePath + GS + outputPlace + argsJoin);
    }

    public void setFrontEventListener(String inputPlace, String htmlEventListener, String modulePath) {
        setFrontEventListener(inputPlace, htmlEventListener, modulePath, null, null);
    }

    public void setMasterPagesEvent(String inputPlace, String htmlEvent, String outputPlace) {
        add("Eu" + inputPlace, htmlEvent + GS + outputPlace);
    }

    public void setMasterPagesEvent(String inputPlace, String htmlEvent) {
        setMasterPagesEvent(inputPlace, htmlEvent, null);
    }

    public void setMasterPagesEventListener(String inputPlace, String htmlEventListener, String outputPlace) {
        add("EU" + inputPlace, htmlEventListener + GS + outputPlace);
    }

    public void setMasterPagesEventListener(String inputPlace, String htmlEventListener) {
        setMasterPagesEventListener(inputPlace, htmlEventListener, null);
    }

    public void setPreventDefaultEvent(String inputPlace, String htmlEvent) {
        add("Ed" + inputPlace, htmlEvent);
    }

    public void setPreventDefaultEventListener(String inputPlace, String htmlEventListener) {
        add("ED" + inputPlace, htmlEventListener);
    }

    public void setStopPropagationEvent(String inputPlace, String htmlEvent) {
        add("Es" + inputPlace, htmlEvent);
    }

    public void setStopPropagationEventListener(String inputPlace, String htmlEventListener) {
        add("ES" + inputPlace, htmlEventListener);
    }

    public void setMethodEvent(String inputPlace, String htmlEvent, String methodName, Object[] args) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("Em" + inputPlace, htmlEvent + GS + methodName + argsJoin);
    }

    public void setMethodEvent(String inputPlace, String htmlEvent, String methodName) {
        setMethodEvent(inputPlace, htmlEvent, methodName, null);
    }

    public void setMethodEventListener(String inputPlace, String htmlEventListener, String methodName, Object[] args) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("EM" + inputPlace, htmlEventListener + GS + methodName + argsJoin);
    }

    public void setMethodEventListener(String inputPlace, String htmlEventListener, String methodName) {
        setMethodEventListener(inputPlace, htmlEventListener, methodName, null);
    }

    public void setModuleMethodEvent(String inputPlace, String htmlEvent, String methodName, Object[] args) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("Ex" + inputPlace, htmlEvent + GS + methodName + argsJoin);
    }

    public void setModuleMethodEvent(String inputPlace, String htmlEvent, String methodName) {
        setModuleMethodEvent(inputPlace, htmlEvent, methodName, null);
    }

    public void setModuleMethodEventListener(String inputPlace, String htmlEventListener, String methodName, Object[] args) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("EX" + inputPlace, htmlEventListener + GS + methodName + argsJoin);
    }

    public void setModuleMethodEventListener(String inputPlace, String htmlEventListener, String methodName) {
        setModuleMethodEventListener(inputPlace, htmlEventListener, methodName, null);
    }

    public void assignConfirmEvent(String inputPlace, String htmlEvent, String text, String type, String title, String okText, String cancelText) {
        add("Ef" + inputPlace, htmlEvent + GS + (text.equals("Are you sure you want to proceed?") ? "" : text) + GS + (type.equals("none") ? "" : type) + GS + (title.equals("Confirm") ? "" : title) + GS + (okText.equals("OK") ? "" : okText) + GS + (cancelText.equals("Cancel") ? "" : cancelText));
    }

    public void assignConfirmEvent(String inputPlace, String htmlEvent) {
        assignConfirmEvent(inputPlace, htmlEvent, "Are you sure you want to proceed?", "none", "Confirm", "OK", "Cancel");
    }

    public void removePostEvent(String inputPlace, String htmlEvent) {
        add("Rp" + inputPlace, htmlEvent);
    }

    public void removePostEventListener(String inputPlace, String htmlEventListener) {
        add("RP" + inputPlace, htmlEventListener);
    }

    public void removeGetEvent(String inputPlace, String htmlEvent) {
        add("Rg" + inputPlace, htmlEvent);
    }

    public void removeGetEventListener(String inputPlace, String htmlEventListener) {
        add("RG" + inputPlace, htmlEventListener);
    }

    public void removePutEvent(String inputPlace, String htmlEvent) {
        add("Rt" + inputPlace, htmlEvent);
    }

    public void removePutEventListener(String inputPlace, String htmlEventListener) {
        add("RT" + inputPlace, htmlEventListener);
    }

    public void removePatchEvent(String inputPlace, String htmlEvent) {
        add("Ra" + inputPlace, htmlEvent);
    }

    public void removePatchEventListener(String inputPlace, String htmlEventListener) {
        add("RA" + inputPlace, htmlEventListener);
    }

    public void removeDeleteEvent(String inputPlace, String htmlEvent) {
        add("Rl" + inputPlace, htmlEvent);
    }

    public void removeDeleteEventListener(String inputPlace, String htmlEventListener) {
        add("RL" + inputPlace, htmlEventListener);
    }

    public void removeOptionsEvent(String inputPlace, String htmlEvent) {
        add("Ro" + inputPlace, htmlEvent);
    }

    public void removeOptionsEventListener(String inputPlace, String htmlEventListener) {
        add("RO" + inputPlace, htmlEventListener);
    }

    public void removeHeadEvent(String inputPlace, String htmlEvent) {
        add("Rh" + inputPlace, htmlEvent);
    }

    public void removeHeadEventListener(String inputPlace, String htmlEventListener) {
        add("RH" + inputPlace, htmlEventListener);
    }

    public void removeSendEvent(String inputPlace, String htmlEvent) {
        add("Rn" + inputPlace, htmlEvent);
    }

    public void removeSendEventListener(String inputPlace, String htmlEventListener) {
        add("RN" + inputPlace, htmlEventListener);
    }

    public void removeCommentEvent(String inputPlace, String htmlEvent) {
        add("Rb" + inputPlace, htmlEvent);
    }

    public void removeCommentEventListener(String inputPlace, String htmlEventListener) {
        add("RB" + inputPlace, htmlEventListener);
    }

    public void removeWasmEvent(String inputPlace, String htmlEvent) {
        add("Ry" + inputPlace, htmlEvent);
    }

    public void removeWasmEventListener(String inputPlace, String htmlEventListener) {
        add("RY" + inputPlace, htmlEventListener);
    }

    public void removeWebSocketEvent(String inputPlace, String htmlEvent) {
        add("Rw" + inputPlace, htmlEvent);
    }

    public void removeWebSocketEventListener(String inputPlace, String htmlEventListener) {
        add("RW" + inputPlace, htmlEventListener);
    }

    public void removeSSEEvent(String inputPlace, String htmlEvent) {
        add("Re" + inputPlace, htmlEvent);
    }

    public void removeSSEEventListener(String inputPlace, String htmlEventListener) {
        add("RE" + inputPlace, htmlEventListener);
    }

    public void removeFrontEvent(String inputPlace, String htmlEvent) {
        add("Rj" + inputPlace, htmlEvent);
    }

    public void removeFrontEventListener(String inputPlace, String htmlEventListener) {
        add("RJ" + inputPlace, htmlEventListener);
    }

    public void removePreventDefaultEvent(String inputPlace, String htmlEvent) {
        add("Rd" + inputPlace, htmlEvent);
    }

    public void removePreventDefaultEventListener(String inputPlace, String htmlEventListener) {
        add("RD" + inputPlace, htmlEventListener);
    }

    public void removeMasterPagesEvent(String inputPlace, String htmlEvent) {
        add("Ru" + inputPlace, htmlEvent);
    }

    public void removeMasterPagesEventListener(String inputPlace, String htmlEventListener) {
        add("RU" + inputPlace, htmlEventListener);
    }

    public void removeStopPropagationEvent(String inputPlace, String htmlEvent) {
        add("Rs" + inputPlace, htmlEvent);
    }

    public void removeStopPropagationEventListener(String inputPlace, String htmlEventListener) {
        add("RS" + inputPlace, htmlEventListener);
    }

    public void removeMethodEvent(String inputPlace, String htmlEvent, String methodName) {
        add("Rm" + inputPlace, htmlEvent + GS + methodName);
    }

    public void removeMethodEventListener(String inputPlace, String htmlEventListener, String methodName) {
        add("RM" + inputPlace, htmlEventListener + GS + methodName);
    }

    public void removeModuleMethodEvent(String inputPlace, String htmlEvent, String methodName) {
        add("Rx" + inputPlace, htmlEvent + GS + methodName);
    }

    public void removeModuleMethodEventListener(String inputPlace, String htmlEventListener, String methodName) {
        add("RX" + inputPlace, htmlEventListener + GS + methodName);
    }

    public void removeConfirmEvent(String inputPlace, String htmlEvent) {
        add("Rf" + inputPlace, htmlEvent);
    }

    // Custom Event
    // This Method Is Compatible With EventListener And May Not Be Compatible With Events Written As Attributes In Some Browsers.
    // Watch: attribute, style, text, children, value
    // Compare: greater, less, equal, notequal, includes, startswith, endswith, matches, changed, inrange, lengthgreater, lengthless, lengthequal
    // Range: Only Use For Compare With inrange Value. Split By Comma ","
    // Key: Only Use For Watch With attribute And style Value
    public void createCustomDOMEvent(String inputPlace, String eventName, String watch, String key, String compare, String value, String range, boolean immediate, String delay) {
        add("eC" + inputPlace, eventName + GS + watch + GS + key + GS + compare + GS + value + GS + range + GS + (immediate ? "1" : "0") + GS + delay);
    }

    public void createCustomDOMEvent(String inputPlace, String eventName, String watch, String key, String compare, String value, String range) {
        createCustomDOMEvent(inputPlace, eventName, watch, key, compare, value, range, false, "0");
    }

    public void createCustomDOMEvent(String inputPlace, String eventName, String watch, String key, String compare, String value, String range, boolean immediate, int delay) {
        createCustomDOMEvent(inputPlace, eventName, watch, key, compare, value, range, immediate, String.valueOf(delay));
    }

    public void enableScrollBottomEvent(boolean enable) {
        add("eb", enable ? "1" : "0");
    }

    public void enableScrollBottomEvent() {
        enableScrollBottomEvent(true);
    }

    public void enableReachedElementEvent(String inputPlace, boolean once, boolean enable) {
        add("er" + inputPlace, (once ? "1" : "0") + GS + (enable ? "1" : "0"));
    }

    public void enableReachedElementEvent(String inputPlace, boolean once) {
        enableReachedElementEvent(inputPlace, once, true);
    }

    // Module
    public void loadModule(String modulePath, String[] methods) {
        if (methods == null) {
            methods = new String[0];
        }
        add("Ml", modulePath + ((methods.length > 0) ? GS + "[" + String.join(String.valueOf(US), methods) : ""));
    }

    public void loadModule(String modulePath) {
        loadModule(modulePath, null);
    }

    public void unloadModule(String modulePath) {
        add("Mu", modulePath);
    }

    public void deleteModuleMethod(String methodName) {
        add("Md", methodName);
    }

    // Unit Testing
    // InputPlace Is Actual, Expected Is Tag/OutputPlace
    public void assertEqual(String inputPlace, String tag) {
        add("At" + inputPlace, tag.replace("\n", "$[ln];"));
    }

    public void assertEqualByOutputPlace(String inputPlace, String outputPlace) {
        add("Ao" + inputPlace, outputPlace);
    }

    // Debug
    public void createDebugger(boolean pause) {
        add("Dc", pause ? "1" : "0");
    }

    public void createDebugger() {
        createDebugger(false);
    }
	
	public void tryBranch() {
		add("tr");
	}

	public void catchBranch() {
		add("ca");
	}

	public void comment(String text) {
		add("//", text);
	}

    // Service Worker
    // To Use Service Worker, You Need To Add The Elanat Dedicated Module (service-worker.js) On The Client Side
    public void serviceWorkerRegister(String path, String scopePath) {
        add("wR", path + GS + scopePath);
    }

    public void serviceWorkerRegister() {
        serviceWorkerRegister(null, null);
    }

    public void serviceWorkerPreCacheStatic(String[] pathList) {
        add("wp", String.join(String.valueOf(GS), pathList));
    }

    public void serviceWorkerDynamicCache(String path, String seconds) {
        add("wc", path + ((seconds != null && !seconds.isEmpty()) ? GS + seconds : ""));
    }

    public void serviceWorkerDynamicCache(String path) {
        serviceWorkerDynamicCache(path, "");
    }

    public void serviceWorkerDynamicCache(String path, int seconds) {
        serviceWorkerDynamicCache(path, seconds > 0 ? String.valueOf(seconds) : "");
    }

    public void serviceWorkerDeleteDynamicCache() {
        add("wd");
    }

    public void serviceWorkerDeleteDynamicCache(String path) {
        add("wd", path);
    }

    public void serviceWorkerDynamicCacheTTLUpdate(String path, String seconds) {
        add("wt", path + ((seconds != null && !seconds.isEmpty()) ? GS + seconds : ""));
    }

    public void serviceWorkerDynamicCacheTTLUpdate(String path) {
        serviceWorkerDynamicCacheTTLUpdate(path, "");
    }

    public void serviceWorkerDynamicCacheTTLUpdate(String path, int seconds) {
        serviceWorkerDynamicCacheTTLUpdate(path, seconds > 0 ? String.valueOf(seconds) : "");
    }

    // Path: Support Wildcard Automatically And Also Support Regex If Use "re:" Before Pattern
    // Type: Type Is Cache Strategy. cachefirst, networkfirst, cacheonly, networkonly, stalerevalidate (Fast From Cache, Updates Simultaneously From The Network)
    // CacheDynamic: If True, Any Successful Network Response For That Route Will Be Stored In The Dynamic Cache
    public void serviceWorkerRouteSet(String path, String type, boolean cacheDynamic) {
        add("wr", path + GS + type + (cacheDynamic ? GS + "1" : ""));
    }

    public void serviceWorkerRouteSet(String path, String type) {
        serviceWorkerRouteSet(path, type, false);
    }

    public void serviceWorkerRouteAlias(String path, String to) {
        add("wa", path + GS + to);
    }

    public void serviceWorkerDeleteRouteAlias(String path) {
        add("wC", path);
    }

    public void serviceWorkerDeleteRouteAlias() {
        serviceWorkerDeleteRouteAlias(null);
    }

    // Delete All Route And Alias
    public void serviceWorkerDeleteRoute() {
        add("wD");
    }

    public void serviceWorkerDeleteRoute(String path) {
        add("wD", path);
    }

    // SSE
    public void disconnectSSE(String path) {
        add("Ds", path);
    }

    public void disconnectSSE() {
        disconnectSSE(null);
    }

    public void disconnectAllSSE() {
        add("Ds");
    }

    // State
    public void addState(String path, String title) {
        add("AS", path + GS + title);
    }

    public void addState() {
        addState(null, null);
    }

    public void saveState(String path, String title) {
        add("As", path + GS + title);
    }

    public void saveState() {
        saveState(null, null);
    }

    public void loadState(String path) {
        add("ls", path);
    }

    public void deleteState(String path) {
        add("DS", path);
    }

    public void deleteState() {
        deleteState(null);
    }

    public void deleteAllState() {
        add("DS", "*");
    }
	
	public void lockQueue(String millisecond) {
		add("lq", millisecond);
	}

	public void lockQueue(int millisecond) {
		lockQueue(String.valueOf(millisecond));
	}

    // Cookie
    public void setCookie(String key, String value, String seconds, String path) {
        add("sC", key + GS + value + GS + seconds + ((path != null && !path.isEmpty()) ? GS + path : ""));
    }

    public void setCookie(String key, String value, String seconds) {
        setCookie(key, value, seconds, null);
    }

    public void setCookie(String key, String value, int seconds, String path) {
        setCookie(key, value, String.valueOf(seconds), path);
    }

    public void setCookie(String key, String value, int seconds) {
        setCookie(key, value, String.valueOf(seconds), null);
    }

    // Save (Session Cache)
    public void saveId(String inputPlace, String key) {
        add("@gi" + inputPlace, key);
    }

    public void saveId(String inputPlace) {
        saveId(inputPlace, ".");
    }

    public void saveName(String inputPlace, String key) {
        add("@gn" + inputPlace, key);
    }

    public void saveName(String inputPlace) {
        saveName(inputPlace, ".");
    }

    public void saveValue(String inputPlace, String key) {
        add("@gv" + inputPlace, key);
    }

    public void saveValue(String inputPlace) {
        saveValue(inputPlace, ".");
    }

    public void saveValueLength(String inputPlace, String key) {
        add("@ge" + inputPlace, key);
    }

    public void saveValueLength(String inputPlace) {
        saveValueLength(inputPlace, ".");
    }

    public void saveClass(String inputPlace, String key) {
        add("@gc" + inputPlace, key);
    }

    public void saveClass(String inputPlace) {
        saveClass(inputPlace, ".");
    }

    public void saveStyle(String inputPlace, String key) {
        add("@gs" + inputPlace, key);
    }

    public void saveStyle(String inputPlace) {
        saveStyle(inputPlace, ".");
    }

    public void saveTitle(String inputPlace, String key) {
        add("@gl" + inputPlace, key);
    }

    public void saveTitle(String inputPlace) {
        saveTitle(inputPlace, ".");
    }

    public void saveLabel(String inputPlace, String key) {
        add("@gA" + inputPlace, key);
    }

    public void saveLabel(String inputPlace) {
        saveLabel(inputPlace, ".");
    }

    public void saveText(String inputPlace, String key) {
        add("@gt" + inputPlace, key);
    }

    public void saveText(String inputPlace) {
        saveText(inputPlace, ".");
    }

    public void saveOuterText(String inputPlace, String key) {
        add("@go" + inputPlace, key);
    }

    public void saveOuterText(String inputPlace) {
        saveOuterText(inputPlace, ".");
    }

    public void saveTextLength(String inputPlace, String key) {
        add("@gg" + inputPlace, key);
    }

    public void saveTextLength(String inputPlace) {
        saveTextLength(inputPlace, ".");
    }

    public void saveAttribute(String inputPlace, String attribute, String key) {
        add("@ga" + inputPlace, key + GS + attribute);
    }

    public void saveAttribute(String inputPlace, String attribute) {
        saveAttribute(inputPlace, attribute, ".");
    }

    public void saveWidth(String inputPlace, String key) {
        add("@gw" + inputPlace, key);
    }

    public void saveWidth(String inputPlace) {
        saveWidth(inputPlace, ".");
    }

    public void saveHeight(String inputPlace, String key) {
        add("@gh" + inputPlace, key);
    }

    public void saveHeight(String inputPlace) {
        saveHeight(inputPlace, ".");
    }

    public void saveReadOnly(String inputPlace, String key) {
        add("@gr" + inputPlace, key);
    }

    public void saveReadOnly(String inputPlace) {
        saveReadOnly(inputPlace, ".");
    }

    public void saveSelectedIndex(String inputPlace, String key) {
        add("@gx" + inputPlace, key);
    }

    public void saveSelectedIndex(String inputPlace) {
        saveSelectedIndex(inputPlace, ".");
    }

    public void saveTextAlign(String inputPlace, String key) {
        add("@gT" + inputPlace, key);
    }

    public void saveTextAlign(String inputPlace) {
        saveTextAlign(inputPlace, ".");
    }

    public void saveNodeLength(String inputPlace, String key) {
        add("@gL" + inputPlace, key);
    }

    public void saveNodeLength(String inputPlace) {
        saveNodeLength(inputPlace, ".");
    }

    public void saveVisible(String inputPlace, String key) {
        add("@gV" + inputPlace, key);
    }

    public void saveVisible(String inputPlace) {
        saveVisible(inputPlace, ".");
    }

    public void saveUrl(String url, boolean fetchScript, String key) {
        add("@gu", key + GS + url + (fetchScript ? GS + "1" : ""));
    }

    public void saveUrl(String url, boolean fetchScript) {
        saveUrl(url, fetchScript, ".");
    }

    public void saveUrl(String url) {
        saveUrl(url, false, ".");
    }

    public void saveIndex(String inputPlace, String key) {
        add("@gI" + inputPlace, key);
    }

    public void saveIndex(String inputPlace) {
        saveIndex(inputPlace, ".");
    }

    public void removeSave(String cacheKey) {
        add("rs", cacheKey);
    }

    public void removeAllSave() {
        add("rs", "*");
    }

    // Calling the SetSave Method Causes Action Control Requests Triggered by Events Using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send Event, to be Temporarily Saved on the Active Page, so the Request will not be Sent to the Server Again.
    public void setSave() {
        add("cs", "*");
    }

    public void addSaveValue(String cacheKey, String value) {
        add("SA", cacheKey + GS + value.replace("\n", "$[ln];"));
    }

    public void insertSaveValue(String cacheKey, String value) {
        add("SI", cacheKey + GS + value.replace("\n", "$[ln];"));
    }

    public void appendSaveValue(String cacheKey, String value) {
        add("SP", cacheKey + GS + value.replace("\n", "$[ln];"));
    }

    public void replaceSaveValue(String cacheKey, String searchValue, String value) {
        add("SR", cacheKey + GS + value.replace("\n", "$[ln];") + GS + searchValue.replace("\n", "$[ln];"));
    }
	
	// Is Regex Replace
	public void setFormatSaveValue(String cacheKey, String regex, String replacement) {
		add("SF", cacheKey + GS + regex + GS + replacement.replace("\n", "$[ln];"));
	}

	// Operator: +, -, *, /, %, //, **
	public void setArithmeticSaveValue(String cacheKey, String operator, String value) {
		add("SM", cacheKey + GS + operator + GS + value);
	}

	public void setArithmeticSaveValue(String cacheKey, String operator, int value) {
		setArithmeticSaveValue(cacheKey, operator, String.valueOf(value));
	}

	// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
	public void setTextOperationSaveValue(String cacheKey, String operation, String value1, String value2) {
		add("ST", cacheKey + GS + operation + GS + value1 + GS + value2);
	}

	public void setTextOperationSaveValue(String cacheKey, String operation, int value1, int value2) {
		setTextOperationSaveValue(cacheKey, operation, String.valueOf(value1), String.valueOf(value2));
	}

    // Cache
    public void cacheId(String inputPlace, String key) {
        add("@ci" + inputPlace, key);
    }

    public void cacheId(String inputPlace) {
        cacheId(inputPlace, ".");
    }

    public void cacheName(String inputPlace, String key) {
        add("@cn" + inputPlace, key);
    }

    public void cacheName(String inputPlace) {
        cacheName(inputPlace, ".");
    }

    public void cacheValue(String inputPlace, String key) {
        add("@cv" + inputPlace, key);
    }

    public void cacheValue(String inputPlace) {
        cacheValue(inputPlace, ".");
    }

    public void cacheValueLength(String inputPlace, String key) {
        add("@ce" + inputPlace, key);
    }

    public void cacheValueLength(String inputPlace) {
        cacheValueLength(inputPlace, ".");
    }

    public void cacheClass(String inputPlace, String key) {
        add("@cc" + inputPlace, key);
    }

    public void cacheClass(String inputPlace) {
        cacheClass(inputPlace, ".");
    }

    public void cacheStyle(String inputPlace, String key) {
        add("@cs" + inputPlace, key);
    }

    public void cacheStyle(String inputPlace) {
        cacheStyle(inputPlace, ".");
    }

    public void cacheTitle(String inputPlace, String key) {
        add("@cl" + inputPlace, key);
    }

    public void cacheTitle(String inputPlace) {
        cacheTitle(inputPlace, ".");
    }

    public void cacheLabel(String inputPlace, String key) {
        add("@cA" + inputPlace, key);
    }

    public void cacheLabel(String inputPlace) {
        cacheLabel(inputPlace, ".");
    }

    public void cacheText(String inputPlace, String key) {
        add("@ct" + inputPlace, key);
    }

    public void cacheText(String inputPlace) {
        cacheText(inputPlace, ".");
    }

    public void cacheOuterText(String inputPlace, String key) {
        add("@co" + inputPlace, key);
    }

    public void cacheOuterText(String inputPlace) {
        cacheOuterText(inputPlace, ".");
    }

    public void cacheTextLength(String inputPlace, String key) {
        add("@cg" + inputPlace, key);
    }

    public void cacheTextLength(String inputPlace) {
        cacheTextLength(inputPlace, ".");
    }

    public void cacheAttribute(String inputPlace, String attribute, String key) {
        add("@ca" + inputPlace, key + GS + attribute);
    }

    public void cacheAttribute(String inputPlace, String attribute) {
        cacheAttribute(inputPlace, attribute, ".");
    }

    public void cacheWidth(String inputPlace, String key) {
        add("@cw" + inputPlace, key);
    }

    public void cacheWidth(String inputPlace) {
        cacheWidth(inputPlace, ".");
    }

    public void cacheHeight(String inputPlace, String key) {
        add("@ch" + inputPlace, key);
    }

    public void cacheHeight(String inputPlace) {
        cacheHeight(inputPlace, ".");
    }

    public void cacheReadOnly(String inputPlace, String key) {
        add("@cr" + inputPlace, key);
    }

    public void cacheReadOnly(String inputPlace) {
        cacheReadOnly(inputPlace, ".");
    }

    public void cacheSelectedIndex(String inputPlace, String key) {
        add("@cx" + inputPlace, key);
    }

    public void cacheSelectedIndex(String inputPlace) {
        cacheSelectedIndex(inputPlace, ".");
    }

    public void cacheTextAlign(String inputPlace, String key) {
        add("@cT" + inputPlace, key);
    }

    public void cacheTextAlign(String inputPlace) {
        cacheTextAlign(inputPlace, ".");
    }

    public void cacheNodeLength(String inputPlace, String key) {
        add("@cL" + inputPlace, key);
    }

    public void cacheNodeLength(String inputPlace) {
        cacheNodeLength(inputPlace, ".");
    }

    public void cacheVisible(String inputPlace, String key) {
        add("@cV" + inputPlace, key);
    }

    public void cacheVisible(String inputPlace) {
        cacheVisible(inputPlace, ".");
    }

    public void cacheUrl(String url, boolean fetchScript, String key) {
        add("@cu", key + GS + url + (fetchScript ? GS + "1" : ""));
    }

    public void cacheUrl(String url, boolean fetchScript) {
        cacheUrl(url, fetchScript, ".");
    }

    public void cacheUrl(String url) {
        cacheUrl(url, false, ".");
    }

    public void cacheIndex(String inputPlace, String key) {
        add("@cI" + inputPlace, key);
    }

    public void cacheIndex(String inputPlace) {
        cacheIndex(inputPlace, ".");
    }

    public void removeCache(String cacheKey) {
        add("rd", cacheKey);
    }

    public void removeAllCache() {
        add("rd", "*");
    }

    // Calling the SetCache Method Causes Action Control Requests Triggered by events using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send event, to be Cached, so the Request will not be Sent to the Server Again.
    public void setCache(String second) {
        add("cd", second);
    }

    public void setCache(int second) {
        setCache(String.valueOf(second));
    }

    public void setCache() {
        add("cd", "*");
    }

    public void addCacheValue(String cacheKey, String value) {
        add("CA", cacheKey + GS + value.replace("\n", "$[ln];"));
    }

    public void insertCacheValue(String cacheKey, String value) {
        add("CI", cacheKey + GS + value.replace("\n", "$[ln];"));
    }

    public void appendCacheValue(String cacheKey, String value) {
        add("CP", cacheKey + GS + value.replace("\n", "$[ln];"));
    }

    public void replaceCacheValue(String cacheKey, String searchValue, String value) {
        add("CR", cacheKey + GS + value.replace("\n", "$[ln];") + GS + searchValue.replace("\n", "$[ln];"));
    }
	
	// Is Regex Replace
	public void setFormatCacheValue(String cacheKey, String regex, String replacement) {
		add("CF", cacheKey + GS + regex + GS + replacement.replace("\n", "$[ln];"));
	}

	// Operator: +, -, *, /, %, //, **
	public void setArithmeticCacheValue(String cacheKey, String operator, String value) {
		add("CM", cacheKey + GS + operator + GS + value);
	}

	public void setArithmeticCacheValue(String cacheKey, String operator, int value) {
		setArithmeticCacheValue(cacheKey, operator, String.valueOf(value));
	}

	// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
	public void setTextOperationCacheValue(String cacheKey, String operation, String value1, String value2) {
		add("CT", cacheKey + GS + operation + GS + value1 + GS + value2);
	}

	public void setTextOperationCacheValue(String cacheKey, String operation, int value1, int value2) {
		setTextOperationCacheValue(cacheKey, operation, String.valueOf(value1), String.valueOf(value2));
	}

    // Call
    public void loadUrl(String inputPlace, String url) {
        add("lu" + inputPlace, url);
    }

    public void runActionControls(String actionControls, boolean withoutWebFormsSection, String index, boolean useCurrentEvent) {
        add("lA", (useCurrentEvent ? "1" : "0") + GS + (withoutWebFormsSection ? "1" : "0") + GS + index + GS + actionControls);
    }

    public void runActionControls(String actionControls) {
        runActionControls(actionControls, true, null, true);
    }

    public void callScript(String scriptText) {
        add("_", scriptText.replace("\n", "$[ln];"));
    }

    public void callMethod(String methodName, Object[] args) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("lm", methodName + argsJoin);
    }

    public void callMethod(String methodName) {
        callMethod(methodName, null);
    }

    public void callModuleMethod(String methodName, Object[] args) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("lM", methodName + argsJoin);
    }

    public void callModuleMethod(String methodName) {
        callModuleMethod(methodName, null);
    }

    public void callPostBack(String formInputPlace, String outputPlace) {
        add("Lp", "1" + GS + formInputPlace + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callPostBack(String formInputPlace) {
        callPostBack(formInputPlace, null);
    }

    public void callCommentBack(String index, String inputPlace, boolean useCurrentEvent) {
        add("LC", (useCurrentEvent ? "1" : "0") + GS + index + GS + inputPlace);
    }

    public void callCommentBack() {
        callCommentBack(null, null, true);
    }

    public void callCommentBack(int index, String inputPlace, boolean useCurrentEvent) {
        callCommentBack(String.valueOf(index), inputPlace, useCurrentEvent);
    }

    public void callWasmBack(String wasmLanguage, String wasmUrl, String methodName, Object[] args, String outputPlace, boolean useCurrentEvent) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("Ly", (useCurrentEvent ? "1" : "0") + GS + wasmLanguage + GS + wasmUrl + GS + methodName + GS + argsJoin + GS + outputPlace);
    }

    public void callWasmBack(String wasmLanguage, String wasmUrl, String methodName) {
        callWasmBack(wasmLanguage, wasmUrl, methodName, null, null, true);
    }

    public void callWebSocketBack(String path, boolean useCurrentEvent) {
        add("Lw", (useCurrentEvent ? "1" : "0") + GS + path);
    }

    public void callWebSocketBack(String path) {
        callWebSocketBack(path, true);
    }

    public void callSSEBack(String path, String outputPlace, boolean useCurrentEvent, boolean shouldReconnect, String reconnectTryTimeout) {
        add("Ls", (useCurrentEvent ? "1" : "0") + GS + path + GS + (shouldReconnect ? "1" : "0") + GS + reconnectTryTimeout + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callSSEBack(String path) {
        callSSEBack(path, null, true, true, "3000");
    }

    public void callSSEBack(String path, String outputPlace, boolean useCurrentEvent, boolean shouldReconnect, int reconnectTryTimeout) {
        callSSEBack(path, outputPlace, useCurrentEvent, shouldReconnect, String.valueOf(reconnectTryTimeout));
    }

    public void callFront(String modulePath, Object[] args, String outputPlace, boolean useCurrentEvent) {
        String argsJoin = "";
        if (args != null) {
            argsJoin = (args.length > 0) ? GS + "[" + String.join(String.valueOf(US), toStringArray(args)) : "";
        }
        add("Lj", (useCurrentEvent ? "1" : "0") + GS + modulePath + GS + outputPlace + argsJoin);
    }

    public void callFront(String modulePath) {
        callFront(modulePath, null, null, true);
    }

    public void callGetBack(String path, String outputPlace, boolean useCurrentEvent) {
        add("Lg", (useCurrentEvent ? "1" : "0") + GS + path + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callGetBack(String path) {
        callGetBack(path, null, true);
    }

    public void callPutBack(String path, String outputPlace, boolean useCurrentEvent) {
        add("Lt", (useCurrentEvent ? "1" : "0") + GS + path + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callPutBack(String path) {
        callPutBack(path, null, true);
    }

    public void callPatchBack(String path, String outputPlace, boolean useCurrentEvent) {
        add("LP", (useCurrentEvent ? "1" : "0") + GS + path + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callPatchBack(String path) {
        callPatchBack(path, null, true);
    }

    public void callDeleteBack(String path, String outputPlace, boolean useCurrentEvent) {
        add("Ld", (useCurrentEvent ? "1" : "0") + GS + path + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callDeleteBack(String path) {
        callDeleteBack(path, null, true);
    }

    public void callHeadBack(String path, boolean useCurrentEvent) {
        add("Lh", (useCurrentEvent ? "1" : "0") + GS + path);
    }

    public void callHeadBack(String path) {
        callHeadBack(path, true);
    }

    public void callOptionsBack(String path, String outputPlace, boolean useCurrentEvent) {
        add("Lo", (useCurrentEvent ? "1" : "0") + GS + path + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callOptionsBack(String path) {
        callOptionsBack(path, null, true);
    }

    public void callSendBack(String path, String method, boolean isMultiPart, String contentType, String data, String outputPlace, boolean useCurrentEvent) {
        add("LS", (useCurrentEvent ? "1" : "0") + GS + path + GS + method + GS + (isMultiPart ? "1" : "0") + GS + contentType + GS + data.replace("\n", "$[ln];") + ((outputPlace != null && !outputPlace.isEmpty()) ? GS + outputPlace : ""));
    }

    public void callSendBack(String path, String method, boolean isMultiPart, String contentType, String data) {
        callSendBack(path, method, isMultiPart, contentType, data, null, true);
    }

    // Update
    public void increase(String inputPlace, float value) {
        add("gt" + inputPlace, "i" + GS + String.valueOf(value));
    }

    public void decrease(String inputPlace, float value) {
        add("gt" + inputPlace, "i" + GS + String.valueOf(value * -1));
    }

    // If You Don't Use Deep Mode, any Tags Inside the Current Tag Will Simply Be Treated as Strings. Deep Mode Does not Remove Inner Elements.
    public void replace(String inputPlace, String value, String newValue, boolean alsoStartTag, boolean deep) {
        add("gt" + inputPlace, "r" + GS + value + GS + newValue + GS + (alsoStartTag ? "1" : "0") + GS + (deep ? "1" : "0"));
    }

    public void replace(String inputPlace, String value, String newValue) {
        replace(inputPlace, value, newValue, false, true);
    }

    // HTML Converts Attribute Names to Lowercase, so they Need to Be Written in Lowercase.
    public void replaceStartTag(String inputPlace, String value, String newValue) {
        add("gt" + inputPlace, "s" + GS + value + GS + newValue);
    }

    // Pre Runner
    public void assignDelay(int milliSecond, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String newName = ":" + milliSecond + ")" + parts[0];
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignDelay(int milliSecond) {
        assignDelay(milliSecond, -1);
    }

    public void assignDelayChange(int milliSecond, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String currentName = parts[0];
        if (currentName.startsWith(":") && currentName.contains(")")) {
            int closingBracket = currentName.indexOf(')');
            currentName = currentName.substring(closingBracket + 1);
        }
        String newName = ":" + milliSecond + ")" + currentName;
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignDelayChange(int milliSecond) {
        assignDelayChange(milliSecond, -1);
    }

    public void assignInterval(int milliSecond, String id, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String newName = "(" + milliSecond + ((id != null && !id.isEmpty()) ? "|" + id : "") + ")" + parts[0];
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignInterval(int milliSecond, String id) {
        assignInterval(milliSecond, id, -1);
    }

    public void assignInterval(int milliSecond) {
        assignInterval(milliSecond, null, -1);
    }

    public void assignIntervalChange(int milliSecond, String id, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String currentName = parts[0];
        if (currentName.startsWith("(") && currentName.contains(")")) {
            int closingBracket = currentName.indexOf(')');
            currentName = currentName.substring(closingBracket + 1);
        }
        String newName = "(" + milliSecond + ((id != null && !id.isEmpty()) ? "|" + id : "") + ")" + currentName;
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignIntervalChange(int milliSecond, String id) {
        assignIntervalChange(milliSecond, id, -1);
    }

    public void assignIntervalChange(int milliSecond) {
        assignIntervalChange(milliSecond, null, -1);
    }

    public void deleteInterval(String id) {
        add("Di", id);
    }

    public void assignRepeat(int count, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String newName = "," + count + ")" + parts[0];
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignRepeat(int count) {
        assignRepeat(count, -1);
    }

    public void assignRepeatChange(int count, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String currentName = parts[0];
        if (currentName.startsWith(",") && currentName.contains(")")) {
            int closingBracket = currentName.indexOf(')');
            currentName = currentName.substring(closingBracket + 1);
        }
        String newName = "," + count + ")" + currentName;
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignRepeatChange(int count) {
        assignRepeatChange(count, -1);
    }

    // Index
    public void startIndex(String name) {
        add("#", name);
    }

    public void startIndex() {
        startIndex("");
    }

    // This Index Is Automatically Run After Changing The Browser History (Back And Forward Buttons)
    public void startState() {
        startIndex("$");
    }

    public void goTo(String line, String repeat) {
        add("&", line + GS + repeat);
    }

    public void goTo(int line, int repeat) {
        goTo(String.valueOf(line), String.valueOf(repeat));
    }

    public void goTo(int line) {
        goTo(line, 1);
    }

    public void goTo(String index, int repeat) {
        add("&", "#" + index + GS + String.valueOf(repeat));
    }

    public void goTo(String index) {
        goTo(index, 1);
    }

    // Start
    public void startTransientDOM(String inputPlace) {
        add("td", inputPlace);
    }

    public void endTransientDOM() {
        add("td", ";");
    }

    // Message
    // Type: warning, problem, help, success, none
    public void alert(String text, String type, String title, String okText) {
        add("Al", text + GS + (type.equals("none") ? "" : type) + GS + (title.equals("Alert") ? "" : title) + GS + (okText.equals("OK") ? "" : okText));
    }

    public void alert(String text) {
        alert(text, "none", "Alert", "OK");
    }

    public void message(String text, String type, String duration) {
        add("me", text + GS + (type.equals("none") ? "" : type) + GS + (duration.equals("0") ? "" : duration));
    }

    public void message(String text) {
        message(text, "none", "0");
    }

    public void message(String text, String type, int duration) {
        message(text, type, String.valueOf(duration));
    }

    public void message(String text, int duration) {
        message(text, "", String.valueOf(duration));
    }

    // Type: log, info, warn, error, debug, trace, group, groupend, table
    public void consoleMessage(String text, String type) {
        add("mc", text.replace("\n", "$[ln];") + (type.equals("log") ? "" : GS + type));
    }

    public void consoleMessage(String text) {
        consoleMessage(text, "log");
    }

    public void consoleMessageAssert(String text, String condition) {
        add("ma", text.replace("\n", "$[ln];") + GS + condition);
    }

    // Enable
    //Calling The EnableWebSocket Or EnableWebSocketOnce Or AddWebSocket Methods Will Cause Any Subsequent Requests (Under WebForms Core Technology) To Operate Under The WebSocket Protocol.
    public void enableWebSocket(boolean enable) {
        add("ew", enable ? "1" : "0");
    }

    public void enableWebSocket() {
        enableWebSocket(true);
    }

    public void enableWebSocketOnce() {
        add("ew", "$");
    }

    public void addWebSocket(String path) {
        add("aw" + path);
    }

    // Disconnected WebSocket
    public void deleteWebSocket(String path) {
        add("dw" + path);
    }

    // Use
    // InputPlace Using Only For form Element
    public void useWebSocket(String inputPlace) {
        add("uw" + inputPlace);
    }

    public void useOnlyChangeUpdate(String inputPlace) {
        add("uo" + inputPlace);
    }

    // Condition And Loop
    // Condition And Loop Supports Brackets and Then
    // Type: warning, problem, help, success, none
    // Interval: Value 0 is Await (if is not True, all Next Action Controls Waiting for it), Value -1 is Sync Check Once (is Support Bracket or Next Action Control), Value > 0 is Async and is Wait Based on Time Repetition Until it Becomes True (Is Support Bracket or Next Action Control, but is not Support Else).
    // Nested Conditions and Nested Loops are Possible.
    public WebForms confirmIsTrueAccept(String text, String type, String title, String okText, String cancelText, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "ct", (text.equals("Are you sure you want to proceed?") ? "" : text) + GS + (type.equals("none") ? "" : type) + GS + (title.equals("Confirm") ? "" : title) + GS + (okText.equals("OK") ? "" : okText) + GS + (cancelText.equals("Cancel") ? "" : cancelText));
        return this;
    }

    public WebForms confirmIsTrueAccept() {
        return confirmIsTrueAccept("Are you sure you want to proceed?", "none", "Confirm", "OK", "Cancel", 100);
    }

    public WebForms confirmIsFalseAccept(String text, String type, String title, String okText, String cancelText, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "cf", (text.equals("Are you sure you want to proceed?") ? "" : text) + GS + (type.equals("none") ? "" : type) + GS + (title.equals("Confirm") ? "" : title) + GS + (okText.equals("OK") ? "" : okText) + GS + (cancelText.equals("Cancel") ? "" : cancelText));
        return this;
    }

    public WebForms confirmIsFalseAccept() {
        return confirmIsFalseAccept("Are you sure you want to proceed?", "none", "Confirm", "OK", "Cancel", 100);
    }

    public WebForms isGreaterThan(String firstValue, String secondValue, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "gt", firstValue + GS + secondValue);
        return this;
    }

    public WebForms isGreaterThan(String firstValue, String secondValue) {
        return isGreaterThan(firstValue, secondValue, -1);
    }

    public WebForms isLessThan(String firstValue, String secondValue, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "lt", firstValue + GS + secondValue);
        return this;
    }

    public WebForms isLessThan(String firstValue, String secondValue) {
        return isLessThan(firstValue, secondValue, -1);
    }

    public WebForms isEqualTo(String firstValue, String secondValue, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "et", firstValue + GS + secondValue);
        return this;
    }

    public WebForms isEqualTo(String firstValue, String secondValue) {
        return isEqualTo(firstValue, secondValue, -1);
    }

    public WebForms isNotEqualTo(String firstValue, String secondValue, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "Nt", firstValue + GS + secondValue);
        return this;
    }

    public WebForms isNotEqualTo(String firstValue, String secondValue) {
        return isNotEqualTo(firstValue, secondValue, -1);
    }

    public WebForms exist(String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "ex", value);
        return this;
    }

    public WebForms exist(String value) {
        return exist(value, -1);
    }

    public WebForms notExist(String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "nx", value);
        return this;
    }

    public WebForms notExist(String value) {
        return notExist(value, -1);
    }

    public WebForms isTrue(String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "tr", value);
        return this;
    }

    public WebForms isTrue(String value) {
        return isTrue(value, -1);
    }

    public WebForms isFalse(String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "fa", value);
        return this;
    }

    public WebForms isFalse(String value) {
        return isFalse(value, -1);
    }

    public WebForms isMatchMedia(String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "mm", value);
        return this;
    }

    public WebForms isMatchMedia(String value) {
        return isMatchMedia(value, -1);
    }

    public WebForms isNotMatchMedia(String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "nm", value);
        return this;
    }

    public WebForms isNotMatchMedia(String value) {
        return isNotMatchMedia(value, -1);
    }

    public WebForms include(String text, String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "In", value + GS + text);
        return this;
    }

    public WebForms include(String text, String value) {
        return include(text, value, -1);
    }

    public WebForms notInclude(String text, String value, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "Nn", value + GS + text);
        return this;
    }

    public WebForms notInclude(String text, String value) {
        return notInclude(text, value, -1);
    }

    public WebForms elementExists(String inputPlace, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "eE", inputPlace);
        return this;
    }

    public WebForms elementExists(String inputPlace) {
        return elementExists(inputPlace, -1);
    }

    public WebForms elementNotExists(String inputPlace, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "nE", inputPlace);
        return this;
    }

    public WebForms elementNotExists(String inputPlace) {
        return elementNotExists(inputPlace, -1);
    }

    public WebForms isRegexMatch(String value, String pattern, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "re", value + GS + pattern);
        return this;
    }

    public WebForms isRegexMatch(String value, String pattern) {
        return isRegexMatch(value, pattern, -1);
    }

    public WebForms isRegexNotMatch(String value, String pattern, int interval) {
        add(((interval >= 0) ? "{(" + interval + ")" : "{") + "rn", value + GS + pattern);
        return this;
    }

    public WebForms isRegexNotMatch(String value, String pattern) {
        return isRegexNotMatch(value, pattern, -1);
    }

    // In: Everything Becomes A JSON List.
    // Key: Creates A Temporary Data In The Browser IndexedDB.
    // Key + "i" Creates A Temporary Data To Maintain The Loop Counter In The Browser IndexedDB.
    public WebForms forEach(String path, String in, String key) {
        add("{fe", path + GS + in + GS + key);
        return this;
    }

    public WebForms forEach(String path, String in) {
        return forEach(path, in, ".");
    }

    public void breakLoop() {
        add(";");
    }

    public WebForms elseBranch() {
        add("}e");
        return this;
    }

    public void startBracket() {
        add("{");
    }

    public void endBracket() {
        add("}");
    }

	// High-Level Methods
    // Used Then In Condition And Loop Methods
    public WebForms then(WebForms newForm) {
        String data = newForm != null ? newForm.getWebFormsData() : null;
        if (data != null && !data.isEmpty()) {
            if (data.contains("\n")) {
                newForm.addToUp("{");
                newForm.add("}");
            }
        }
        appendForm(newForm);
        return this;
    }

    public WebForms then(Runnable configure) {
        WebForms newForm = new WebForms();
        configure.run();
        String data = newForm.getWebFormsData();
        if (data != null && !data.isEmpty()) {
            if (data.contains("\n")) {
                newForm.addToUp("{");
                newForm.add("}");
            }
        }
        appendForm(newForm);
        return this;
    }

    public WebForms repeat(WebForms newForm, int repeat) {
        if (newForm == null) {
            return this;
        }
        String bodyData = newForm.getWebFormsData();
        if (bodyData == null || bodyData.isEmpty()) {
            return this;
        }
        int startLine = bodyData.split("\n", -1).length * -1;
        appendForm(newForm);
        goTo(startLine, repeat - 1);
        return this;
    }

    public WebForms repeat(WebForms newForm, int repeat, String index) {
        if (newForm == null) {
            return this;
        }
        goTo(index);
        startIndex(index);
        String bodyData = newForm.getWebFormsData();
        if (bodyData == null || bodyData.isEmpty()) {
            return this;
        }
        appendForm(newForm);
        if (index == null || index.isEmpty()) {
            int indexNumber = -1;
            for (String x : getWebFormsData().split("\n", -1)) {
                if (x.startsWith("#")) {
                    indexNumber++;
                }
            }
            goTo(String.valueOf(indexNumber), repeat - 1);
        } else {
            goTo(index, repeat - 1);
        }
        return this;
    }

    public WebForms repeat(Consumer<WebForms> configure, int repeat) {
        WebForms newForm = new WebForms();
        configure.accept(newForm);
        return repeat(newForm, repeat);
    }

    public WebForms repeat(Consumer<WebForms> configure, int repeat, String index) {
        WebForms newForm = new WebForms();
        configure.accept(newForm);
        return repeat(newForm, repeat, index);
    }
	
	public WebForms isole(WebForms newForm, String inputPlace) {
		if (newForm == null) {
			return this;
		}

		String bodyData = newForm.getWebFormsData();

		if (bodyData == null || bodyData.isEmpty()) {
			return this;
		}

		WebForms form = new WebForms();
		form.startTransientDOM(inputPlace);

		appendForm(form);
		newForm.endTransientDOM();
		appendForm(newForm);

		return this;
	}

	public WebForms isole(Consumer<WebForms> configure, String inputPlace) {
		WebForms newForm = new WebForms();
		configure.accept(newForm);
		return isole(newForm, inputPlace);
	}

	// The Render Method is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
	// This Method Utilizes the Transient DOM; Therefore, When Selecting InputPlaces, You Must Consider the Root of the Selected InputPlace Within the Method.
	public WebForms render(WebForms newForm, String inputPlace, String key, boolean permanent) {
		if (newForm == null) {
			return this;
		}

		String bodyData = newForm.getWebFormsData();

		if (bodyData == null || bodyData.isEmpty()) {
			return this;
		}

		if (key == null || key.isEmpty()) {
			key = inputPlace;
		}

		WebForms form = new WebForms();

		form.exist(permanent ? Fetch.cache(key) : Fetch.save(key));
			form.rollback(inputPlace, key, permanent);
		form.elseBranch();
			form.snapshot(inputPlace, key, permanent);

		form.startTransientDOM(inputPlace);

		appendForm(form);
		newForm.endTransientDOM();
		appendForm(newForm);

		return this;
	}

	public WebForms render(WebForms newForm, String inputPlace) {
		return render(newForm, inputPlace, "", false);
	}

	public WebForms render(WebForms newForm, String inputPlace, String key) {
		return render(newForm, inputPlace, key, false);
	}

	public WebForms render(Consumer<WebForms> configure, String inputPlace, String key, boolean permanent) {
		WebForms newForm = new WebForms();
		configure.accept(newForm);
		return render(newForm, inputPlace, key, permanent);
	}

	public WebForms render(Consumer<WebForms> configure, String inputPlace) {
		return render(configure, inputPlace, "", false);
	}

	public WebForms render(Consumer<WebForms> configure, String inputPlace, String key) {
		return render(configure, inputPlace, key, false);
	}

    // Async
    // It Supports Brackets and Then
    public WebForms async() {
        add("{(a)");
        return this;
    }

    public void delay(String milliSecond) {
        add("De", milliSecond);
    }

    public void delay(int milliSecond) {
        delay(String.valueOf(milliSecond));
    }

    // Option
    public void changeOption(String name, String value) {
        add("co", name + GS + value);
    }

    public void resetOption() {
        add("ro");
    }

    public void resetOption(String name) {
        add("ro", name);
    }

    // Format Storage
    public void createFormatStorage(String key, String data) {
        add(".C", key + GS + data);
    }

    public void deleteFormatStorage(String key) {
        add(".D", key);
    }

    public void addJSON(String key, String path, String value) {
        add(".a", key + GS + "j" + GS + value + GS + path);
    }

    // Name: For Support Attribute, Set Double At Sign (@@) Before Name.
    public void addXML(String key, String path, String name, String value) {
        add(".a", key + GS + "x" + GS + name + GS + value + GS + path);
    }

    public void addXML(String key, String path, String name) {
        addXML(key, path, name, null);
    }

    public void addINI(String key, String path, String value, boolean isINILike) {
        add(".a", key + GS + "i" + GS + (isINILike ? "1" : "0") + GS + value + GS + path);
    }

    public void addINI(String key, String path, String value) {
        addINI(key, path, value, false);
    }

    public void addTextLine(String key, String line, String text) {
        add(".a", key + GS + "t" + GS + text + GS + line);
    }

    public void addTextLine(String key, int line, String text) {
        addTextLine(key, String.valueOf(line), text);
    }

    public void addVariable(String key, String value) {
        add(".a", key + GS + "v" + GS + value);
    }

    public void updateJSON(String key, String path, String value) {
        add(".u", key + GS + "j" + GS + value + GS + path);
    }

    public void updateXML(String key, String path, String value) {
        add(".u", key + GS + "x" + GS + value + GS + path);
    }

    public void updateINI(String key, String path, String value, boolean isINILike) {
        add(".u", key + GS + "i" + GS + (isINILike ? "1" : "0") + GS + value + GS + path);
    }

    public void updateINI(String key, String path, String value) {
        updateINI(key, path, value, false);
    }

    public void updateTexLine(String key, String line, String text) {
        add(".u", key + GS + "t" + GS + text + GS + line);
    }

    public void updateTexLine(String key, int line, String text) {
        updateTexLine(key, String.valueOf(line), text);
    }

    public void updateVariable(String key, String value) {
        add(".u", key + GS + "v" + GS + value);
    }

    public void increaseVariable(String key, String value) {
        add(".i", key + GS + "v" + GS + value);
    }

    public void increaseVariable(String key, int value) {
        increaseVariable(key, String.valueOf(value));
    }

    public void decreaseVariable(String key, int value) {
        increaseVariable(key, String.valueOf(value * -1));
    }

    public void deleteJSON(String key, String path) {
        add(".d", key + GS + "j" + GS + path);
    }

    public void deleteXML(String key, String path) {
        add(".d", key + GS + "x" + GS + path);
    }

    public void deleteINI(String key, String path, boolean isINILike) {
        add(".d", key + GS + "i" + GS + (isINILike ? "1" : "0") + GS + path);
    }

    public void deleteINI(String key, String path) {
        deleteINI(key, path, false);
    }

    public void deleteTextLine(String key, String line) {
        add(".d", key + GS + "t" + GS + line);
    }

    public void deleteTextLine(String key, int line) {
        deleteTextLine(key, String.valueOf(line));
    }

    public void deleteVariable(String key) {
        add(".d", key + GS + "v");
    }

    // Template Engine
    // Pattern Example: {{value}}, ((value)), *value*, $value;
    public void bindJSONToTemplate(String inputPlace, String jsonText, String path, String pattern, boolean alsoStartTag) {
        add("Tj" + inputPlace, jsonText + GS + path + GS + pattern + GS + (alsoStartTag ? "1" : "0"));
    }

    public void bindJSONToTemplate(String inputPlace, String jsonText, String path, String pattern) {
        bindJSONToTemplate(inputPlace, jsonText, path, pattern, true);
    }

    // Because XML Elements Are Lowercased, Placeholders Must Use Lowercase Names.
    public void bindXMLToTemplate(String inputPlace, String xmlText, String path, String pattern, boolean alsoStartTag) {
        add("Tx" + inputPlace, xmlText + GS + path + GS + pattern + GS + (alsoStartTag ? "1" : "0"));
    }

    public void bindXMLToTemplate(String inputPlace, String xmlText, String path, String pattern) {
        bindXMLToTemplate(inputPlace, xmlText, path, pattern, true);
    }

    public void bindINIToTemplate(String inputPlace, String iniText, String path, String pattern, boolean alsoStartTag) {
        add("Ti" + inputPlace, iniText + GS + path + GS + pattern + GS + (alsoStartTag ? "1" : "0"));
    }

    public void bindINIToTemplate(String inputPlace, String iniText, String path, String pattern) {
        bindINIToTemplate(inputPlace, iniText, path, pattern, true);
    }

    // Inject
    // Need Add @: to First of String
    public String inject(String value) {
        return "$[" + value + "];";
    }

    // Action Control
    public void replaceActionControl(String searchValue, String value, boolean addingToUp) {
        if (addingToUp) {
            addToUp("rE", searchValue + GS + value);
        } else {
            add("rE", searchValue + GS + value);
        }
    }

    public void replaceActionControl(String searchValue, String value) {
        replaceActionControl(searchValue, value, false);
    }

    public void assignReplace(String searchValue, String value, int index) {
        String currentLine = getLineByIndex(index);
        if (currentLine == null || currentLine.isEmpty()) {
            return;
        }
        String[] parts = currentLine.split("=", 2);
        String newName = ";" + searchValue + GS + value + GS + parts[0];
        String newValue = parts.length > 1 ? parts[1] : "";
        updateLineByIndex(index, newName, newValue);
    }

    public void assignReplace(String searchValue, String value) {
        assignReplace(searchValue, value, -1);
    }

    // Hash And Checksum
    public void setHash() {
        add("SH");
    }

    public void setChecksum() {
        add("CS");
    }

    public String checksumCalculation(String text) {
        int sum = 0;
        int mod = 65536;
        int shift = 5;
        for (int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            sum = ((sum << shift) | (sum >>> (16 - shift))) ^ c;
            sum %= mod;
        }
        return String.valueOf(sum);
    }

    public String getChecksum() {
        return checksumCalculation(getWebFormsData());
    }

    // Get
    public String getWebFormsData() {
        return webFormsData.toString();
    }

    public String getFormsActionData() {
        if (webFormsData.length() == 0) {
            return "";
        }
        return webFormsData.toString();
    }

    public String response() {
        return "[web-forms]\n" + getFormsActionData();
    }

    public String getFormsActionDataLineBreak() {
        if (webFormsData.length() == 0) {
            return "";
        }
        String data = webFormsData.toString();
        String processedData = data.replace("\"", "$[dq];");
        return processedData.replace("\n", "$[sln];");
    }

    // Export
    public String exportToHtmlComment(boolean addLine) {
        String response = response().replace("--", "$[dd];");
        if (response.endsWith("-")) {
            response = response.substring(0, response.length() - 1) + "$[da];";
        }
        return (addLine ? "\n" : "") + "<!--" + response + "-->";
    }

    public String exportToHtmlComment() {
        return exportToHtmlComment(false);
    }

    // Using it for SSE Response
    public String exportToLineBreak(String src) {
        return "[web-forms]$[sln];" + getFormsActionDataLineBreak();
    }

    public String exportToLineBreak() {
        return exportToLineBreak(null);
    }

    public void appendForm(WebForms form) {
        if (form == null) {
            return;
        }
        String otherData = form.getWebFormsData();
        if (otherData != null && !otherData.isEmpty()) {
            if (webFormsData.length() > 0) {
                webFormsData.append('\n');
            }
            webFormsData.append(otherData);
        }
    }

    public void clean() {
        webFormsData.setLength(0);
    }

    private String[] toStringArray(Object[] args) {
        String[] result = new String[args.length];
        for (int i = 0; i < args.length; i++) {
            result[i] = args[i] != null ? args[i].toString() : "";
        }
        return result;
    }

    @FunctionalInterface
    public interface Consumer<T> {
        void accept(T t);
    }
}
