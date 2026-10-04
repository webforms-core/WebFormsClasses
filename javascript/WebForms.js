// WebForms.js 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

export class WebForms {
    static GS = "\x1D";
    static US = "\x1F";

    constructor() {
        this._webFormsData = "";
    }

    _add(name, value = null) {
        if (this._webFormsData.length > 0) this._webFormsData += "\n";
        this._webFormsData += name;
        if (value !== null) this._webFormsData += "=" + value;
    }

    _addToUp(name, value = null) {
        let line = name + (value !== null ? "=" + value : "");
        if (this._webFormsData.length > 0) line += "\n";
        this._webFormsData = line + this._webFormsData;
    }

    _getLineByIndex(index) {
        if (this._webFormsData.length === 0) return "";
        const lines = this._webFormsData.split("\n");
        if (index < 0) index = lines.length + index;
        if (index < 0 || index >= lines.length) return "";
        return lines[index];
    }

    _updateLineByIndex(index, name, value = null) {
        if (this._webFormsData.length === 0) return;
        const lines = this._webFormsData.split("\n");
        if (index < 0) index = lines.length + index;
        if (index < 0 || index >= lines.length) return;
        lines[index] = name + ((value !== null && value !== "") ? "=" + value : "");
        this._webFormsData = lines.join("\n");
    }

    // For Extension
    addLine(name, value) { this._add(name, value); }

    // Add
    // Creates the Data if it does not exist; otherwise, Appends the New Value to the Existing Value.
    addId(inputPlace, id) { this._add("ai" + inputPlace, id); }
    addName(inputPlace, name) { this._add("an" + inputPlace, name); }
    addValue(inputPlace, value) { this._add("av" + inputPlace, value); }
    addClass(inputPlace, className) { this._add("ac" + inputPlace, className); }
    
    addStyle(inputPlace, arg1, arg2 = null) {
        if (arguments.length === 3) this._add("as" + inputPlace, arg1 + ":" + arg2);
        else this._add("as" + inputPlace, arg1);
    }
    
    addOptionTag(inputPlace, text, value, selected = false) { this._add("ao" + inputPlace, value + WebForms.GS + text + (selected ? WebForms.GS + "1" : "")); }
    addCheckBoxTag(inputPlace, text, value, checked = false) { this._add("ak" + inputPlace, value + WebForms.GS + text + (checked ? WebForms.GS + "1" : "")); }
    addTitle(inputPlace, title) { this._add("al" + inputPlace, title); }
    addLabel(inputPlace, label) { this._add("aA" + inputPlace, label); }
    addText(inputPlace, text) { this._add("at" + inputPlace, text.replace(/\n/g, "$[ln];")); }
    addTextToUp(inputPlace, text) { this._add("pt" + inputPlace, text.replace(/\n/g, "$[ln];")); }
    
    addAttribute(inputPlace, attribute, value = "", splitter = "") {
        this._add("aa" + inputPlace, attribute + WebForms.GS + (splitter !== "" ? splitter : "") + ((value !== null && value !== "") ? WebForms.GS + value : ""));
    }
    
    addTag(inputPlace, tagName, id = "") { this._add("nt" + inputPlace, tagName + (id !== "" ? WebForms.GS + id : "")); }
    addTagToUp(inputPlace, tagName, id = "") { this._add("ut" + inputPlace, tagName + (id !== "" ? WebForms.GS + id : "")); }
    addTagBefore(inputPlace, tagName, id = "") { this._add("bt" + inputPlace, tagName + (id !== "" ? WebForms.GS + id : "")); }
    addTagAfter(inputPlace, tagName, id = "") { this._add("ft" + inputPlace, tagName + (id !== "" ? WebForms.GS + id : "")); }
    addHidden(inputPlace, name, value, id = "") { this._add("ah" + inputPlace, name + WebForms.GS + value + (id !== "" ? WebForms.GS + id : "")); }

    // Set
    // Creates the Data if it does not exist; otherwise, Replaces the Existing Value with the New Value.
    setId(inputPlace, id) { this._add("si" + inputPlace, id); }
    setName(inputPlace, name) { this._add("sn" + inputPlace, name); }
    setValue(inputPlace, value) { this._add("sv" + inputPlace, value); }
    setClass(inputPlace, className) { this._add("sc" + inputPlace, className); }
    
    setStyle(inputPlace, arg1, arg2 = null) {
        if (arguments.length === 3) this._add("ss" + inputPlace, arg1 + ":" + arg2);
        else this._add("ss" + inputPlace, arg1);
    }
    
    setOptionTag(inputPlace, text, value, selected = false) { this._add("so" + inputPlace, value + WebForms.GS + text + (selected ? WebForms.GS + "1" : "")); }
    setChecked(inputPlace, checked = false) { this._add("sk" + inputPlace, checked ? "1" : "0"); }
    setCheckBoxTag(inputPlace, text, value, checked = false) { this._add("sk" + inputPlace, value + WebForms.GS + text + (checked ? WebForms.GS + "1" : "")); }
    setTitle(inputPlace, title) { this._add("sl" + inputPlace, title); }
    setLabel(inputPlace, label) { this._add("sA" + inputPlace, label); }
    setText(inputPlace, text) { this._add("st" + inputPlace, text.replace(/\n/g, "$[ln];")); }
    setAttribute(inputPlace, attribute, value = "") { this._add("sa" + inputPlace, attribute + WebForms.GS + ((value !== null && value !== "") ? WebForms.GS + value : "")); }
    
    setWidth(inputPlace, width) {
        this._add("sw" + inputPlace, typeof width === "number" ? width + "px" : width);
    }
    
    setHeight(inputPlace, height) {
        this._add("sh" + inputPlace, typeof height === "number" ? height + "px" : height);
    }
    
    setBackgroundColor(inputPlace, color) { this._add("bc" + inputPlace, color); }
    setTextColor(inputPlace, color) { this._add("tc" + inputPlace, color); }
    setFontName(inputPlace, name) { this._add("fn" + inputPlace, name); }
    
    setFontSize(inputPlace, size) {
        this._add("fs" + inputPlace, typeof size === "number" ? size + "px" : size);
    }
    
    setFontBold(inputPlace, bold) { this._add("fb" + inputPlace, bold ? "1" : "0"); }
    setVisible(inputPlace, visible) { this._add("vi" + inputPlace, visible ? "1" : "0"); }
    setTextAlign(inputPlace, align) { this._add("ta" + inputPlace, align); }
    setReadOnly(inputPlace, readOnly) { this._add("sr" + inputPlace, readOnly ? "1" : "0"); }
    setDisabled(inputPlace, disabled) { this._add("sd" + inputPlace, disabled ? "1" : "0"); }
    setFocus(inputPlace, focus) { this._add("sf" + inputPlace, focus ? "1" : "0"); }
    setMinLength(inputPlace, length) { this._add("mn" + inputPlace, length); }
    setMaxLength(inputPlace, length) { this._add("mx" + inputPlace, length); }   
    setSelectedValue(inputPlace, value) { this._add("ts" + inputPlace, value); }
    setSelectedIndex(inputPlace, index) { this._add("ti" + inputPlace, index); }
    setCheckedValue(inputPlace, value, checked) { this._add("ks" + inputPlace, value + WebForms.GS + (checked ? "1" : "0")); }
    setCheckedIndex(inputPlace, index, checked) { this._add("ki" + inputPlace, index + WebForms.GS + (checked ? "1" : "0")); }
	setCustomValidity(inputPlace, text) { this._add("cv" + inputPlace, text.replace(/\n/g, "$[ln];")); }

    // Insert
    // Creates the Data only if it does not exist; otherwise, does nothing.
    insertId(inputPlace, id) { this._add("ii" + inputPlace, id); }
    insertName(inputPlace, name) { this._add("in" + inputPlace, name); }
    insertValue(inputPlace, value) { this._add("iv" + inputPlace, value); }
    insertClass(inputPlace, className) { this._add("ic" + inputPlace, className); }
    
    insertStyle(inputPlace, arg1, arg2 = null) {
        if (arguments.length === 3) this._add("is" + inputPlace, arg1 + ":" + arg2);
        else this._add("is" + inputPlace, arg1);
    }
    
    insertOptionTag(inputPlace, text, value, selected = false) { this._add("io" + inputPlace, value + WebForms.GS + text + (selected ? WebForms.GS + "1" : "")); }
    insertCheckBoxTag(inputPlace, text, value, checked = false) { this._add("ik" + inputPlace, value + WebForms.GS + text + (checked ? WebForms.GS + "1" : "")); }
    insertTitle(inputPlace, title) { this._add("il" + inputPlace, title); }
    insertLabel(inputPlace, label) { this._add("iA" + inputPlace, label); }
    insertText(inputPlace, text) { this._add("it" + inputPlace, text.replace(/\n/g, "$[ln];")); }
    
    insertAttribute(inputPlace, attribute, value = "", splitter = "") {
        this._add("ia" + inputPlace, attribute + WebForms.GS + (splitter !== "" ? splitter : "") + ((value !== null && value !== "") ? WebForms.GS + value : ""));
    }

    // Delete
    deleteId(inputPlace) { this._add("di" + inputPlace); }
    deleteName(inputPlace) { this._add("dn" + inputPlace); }
    deleteValue(inputPlace) { this._add("dv" + inputPlace); }
    deleteClass(inputPlace, className) { this._add("dc" + inputPlace, className); }
    deleteStyle(inputPlace, styleName) { this._add("ds" + inputPlace, styleName); }
    deleteOptionTag(inputPlace, value) { this._add("do" + inputPlace, value); }
    deleteAllOptionTag(inputPlace) { this._add("do" + inputPlace, "*"); }
    deleteCheckBoxTag(inputPlace, value) { this._add("dk" + inputPlace, value); }
    deleteAllCheckBoxTag(inputPlace) { this._add("dk" + inputPlace, "*"); }
    deleteTitle(inputPlace) { this._add("dl" + inputPlace); }
    deleteLabel(inputPlace) { this._add("dA" + inputPlace); }
    deleteText(inputPlace) { this._add("dt" + inputPlace); }
    deleteAttribute(inputPlace, attribute) { this._add("da" + inputPlace, attribute); }
    delete(inputPlace) { this._add("de" + inputPlace); }
    deleteParent(inputPlace) { this._add("dp" + inputPlace); }

    // Tag Transformation
    swapTag(inputPlace, outputPlace) { this._add("sp" + inputPlace, outputPlace); }
    setReflect(inputPlace, tag) { this._add("sR" + inputPlace, tag); }
    setReflectByOutputPlace(inputPlace, outputPlace) { this._add("iR" + inputPlace, outputPlace); }
    setMorph(inputPlace, tag) { this._add("sM" + inputPlace, tag); }
    setMorphByOutputPlace(inputPlace, outputPlace) { this._add("iM" + inputPlace, outputPlace); }
	// The Snapshot and Rollback Mechanism is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
	// inputPlace: Only One Tag
	snapshot(inputPlace, key = "", permanent = false) {this._add("rS" + inputPlace, (key !== "" ? key : inputPlace) + (permanent ? WebForms.GS + "1" : "")); }
	rollback(inputPlace, key = "", permanent = false) {this._add("rB" + inputPlace, (key !== "" ? key : inputPlace) + (permanent ? WebForms.GS + "1" : "")); }

    // Browser
    changeUrl(url) { this._add("cu", url); }
    setHeadTitle(title) { this._add("ht", title); }
    clipboardWriteText(text) { this._add("nw", text); }
    scrollTo(x, y) { this._add("ws", x + WebForms.GS + y); }
    historyGo(steps) { this._add("wg", steps); }
    reloadPage() { this._add("lr"); }
    redirect(path) { this._add("lh", path); }

    // Increase
    increaseMinLength(inputPlace, value) { this._add("+n" + inputPlace, value); }
    increaseMaxLength(inputPlace, value) { this._add("+x" + inputPlace, value); }
    increaseFontSize(inputPlace, value) { this._add("+f" + inputPlace, value); }
    increaseWidth(inputPlace, value) { this._add("+w" + inputPlace, value); }
    increaseHeight(inputPlace, value) { this._add("+h" + inputPlace, value); }
    increaseValue(inputPlace, value) { this._add("+v" + inputPlace, value); }

    // Decrease
    decreaseMinLength(inputPlace, value) { this._add("-n" + inputPlace, value); }
    decreaseMaxLength(inputPlace, value) { this._add("-x" + inputPlace, value); }
    decreaseFontSize(inputPlace, value) { this._add("-f" + inputPlace, value); }
    decreaseWidth(inputPlace, value) { this._add("-w" + inputPlace, value); }
    decreaseHeight(inputPlace, value) { this._add("-h" + inputPlace, value); }
    decreaseValue(inputPlace, value) { this._add("-v" + inputPlace, value); }

    // Event
    // ConstructorName: mouseevent, keyboardevent, uievent, focusevent, inputevent, event
    // All Method in "Event" Section Only Support Dynamic Args Once. To Support Invoking Dynamic Arguments on a Momentary Basis, Use "EventListener" Section Methods.
    triggerEvent(inputPlace, htmlEventListener, constructorName = null) { this._add("TE" + inputPlace, htmlEventListener + (constructorName !== null ? WebForms.GS + constructorName : "")); }
    
    setPostEvent(inputPlace, htmlEvent, arg1 = null) {
        if (arguments.length === 3) this._add("Ep" + inputPlace, htmlEvent + WebForms.GS + arg1);
        else this._add("Ep" + inputPlace, htmlEvent);
    }
    setPostEventAddView(inputPlace, htmlEvent) { this._add("Ep" + inputPlace, htmlEvent + WebForms.GS + "+"); }
    
    setPostEventListener(inputPlace, htmlEventListener, arg1 = null) {
        if (arguments.length === 3) this._add("EP" + inputPlace, htmlEventListener + WebForms.GS + arg1);
        else this._add("EP" + inputPlace, htmlEventListener);
    }
    setPostEventListenerAddView(inputPlace, htmlEventListener) { this._add("EP" + inputPlace, htmlEventListener + WebForms.GS + "+"); }
    
    setGetEvent(inputPlace, htmlEvent, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("Eg" + inputPlace, htmlEvent + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("Eg" + inputPlace, htmlEvent + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setGetEventListener(inputPlace, htmlEventListener, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("EG" + inputPlace, htmlEventListener + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("EG" + inputPlace, htmlEventListener + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setPutEvent(inputPlace, htmlEvent, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("Et" + inputPlace, htmlEvent + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("Et" + inputPlace, htmlEvent + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setPutEventListener(inputPlace, htmlEventListener, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("ET" + inputPlace, htmlEventListener + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("ET" + inputPlace, htmlEventListener + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setPatchEvent(inputPlace, htmlEvent, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("Ea" + inputPlace, htmlEvent + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("Ea" + inputPlace, htmlEvent + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setPatchEventListener(inputPlace, htmlEventListener, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("EA" + inputPlace, htmlEventListener + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("EA" + inputPlace, htmlEventListener + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setDeleteEvent(inputPlace, htmlEvent, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("El" + inputPlace, htmlEvent + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("El" + inputPlace, htmlEvent + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setDeleteEventListener(inputPlace, htmlEventListener, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("EL" + inputPlace, htmlEventListener + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("EL" + inputPlace, htmlEventListener + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setOptionsEvent(inputPlace, htmlEvent, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("Eo" + inputPlace, htmlEvent + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("Eo" + inputPlace, htmlEvent + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setOptionsEventListener(inputPlace, htmlEventListener, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            const path = arg1 !== null ? arg1 : "#";
            this._add("EO" + inputPlace, htmlEventListener + WebForms.GS + path);
        } else {
            const path = arg2 !== null ? arg2 : "#";
            this._add("EO" + inputPlace, htmlEventListener + WebForms.GS + path + WebForms.GS + arg1);
        }
    }
    
    setHeadEvent(inputPlace, htmlEvent, path = null) { this._add("Eh" + inputPlace, htmlEvent + WebForms.GS + (path !== null ? path : "#")); }
    setHeadEventListener(inputPlace, htmlEventListener, path = null) { this._add("EH" + inputPlace, htmlEventListener + WebForms.GS + (path !== null ? path : "#")); }
    
    // IsMultiPart: If this value is true, the data will be sent based on the Form and with the "content" key.
    setSendEvent(inputPlace, htmlEvent, data, path = null, method = "POST", isMultiPart = false, contentType = "text/plain", outputPlace = null) {
        this._add("En" + inputPlace, htmlEvent + WebForms.GS + data.replace(/\n/g, "$[ln];").replace(/"/g, "$[dq];").replace(/'/g, "$[sq];") + WebForms.GS + (path !== null ? path : "#") + WebForms.GS + method + WebForms.GS + (isMultiPart ? "1" : "0") + WebForms.GS + contentType + WebForms.GS + outputPlace);
    }
    
    setSendEventListener(inputPlace, htmlEventListener, data, path = null, method = "POST", isMultiPart = false, contentType = "text/plain", outputPlace = null) {
        this._add("EN" + inputPlace, htmlEventListener + WebForms.GS + data.replace(/\n/g, "$[ln];") + WebForms.GS + (path !== null ? path : "#") + WebForms.GS + method + WebForms.GS + (isMultiPart ? "1" : "0") + WebForms.GS + contentType + WebForms.GS + outputPlace);
    }
    
    setCommentEvent(inputPlace, htmlEvent, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            this._add("Eb" + inputPlace, htmlEvent + WebForms.GS + arg1 + WebForms.GS + "");
        } else {
            this._add("Eb" + inputPlace, htmlEvent + WebForms.GS + arg1 + WebForms.GS + arg2);
        }
    }
    
    setCommentEventListener(inputPlace, htmlEventListener, arg1 = null, arg2 = null) {
        if (arguments.length === 3) {
            this._add("EB" + inputPlace, htmlEventListener + WebForms.GS + arg1 + WebForms.GS + "");
        } else {
            this._add("EB" + inputPlace, htmlEventListener + WebForms.GS + arg1 + WebForms.GS + arg2);
        }
    }
    
    setWasmEvent(inputPlace, htmlEvent, wasmLanguage, wasmUrl, methodName, args = null, outputPlace = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = "[" + args.map(String).join(WebForms.US);
        this._add("Ey" + inputPlace, htmlEvent + WebForms.GS + wasmLanguage + WebForms.GS + wasmUrl + WebForms.GS + methodName + WebForms.GS + argsJoin + WebForms.GS + outputPlace);
    }
    
    setWasmEventListener(inputPlace, htmlEventListener, wasmLanguage, wasmUrl, methodName, args = null, outputPlace = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = "[" + args.map(String).join(WebForms.US);
        this._add("EY" + inputPlace, htmlEventListener + WebForms.GS + wasmLanguage + WebForms.GS + wasmUrl + WebForms.GS + methodName + WebForms.GS + argsJoin + WebForms.GS + outputPlace);
    }
    
    setWebSocketEvent(inputPlace, htmlEvent, path) { this._add("Ew" + inputPlace, htmlEvent + WebForms.GS + path); }
    setWebSocketEventListener(inputPlace, htmlEventListener, path) { this._add("EW" + inputPlace, htmlEventListener + WebForms.GS + path); }
    
    setSSEEvent(inputPlace, htmlEvent, path, arg1 = true, arg2 = 3000, arg3 = null) {
        let outputPlace = null, shouldReconnect = true, reconnectTryTimeout = 3000;
        if (arguments.length === 6) { outputPlace = arg1; shouldReconnect = arg2; reconnectTryTimeout = arg3; }
        else { shouldReconnect = arg1; reconnectTryTimeout = arg2; }
        let val = htmlEvent + WebForms.GS + path + WebForms.GS + (shouldReconnect ? "1" : "0") + WebForms.GS + reconnectTryTimeout;
        if (outputPlace !== null) val += WebForms.GS + outputPlace;
        this._add("Ee" + inputPlace, val);
    }
    
    setSSEEventListener(inputPlace, htmlEventListener, path, arg1 = true, arg2 = 3000, arg3 = null) {
        let outputPlace = null, shouldReconnect = true, reconnectTryTimeout = 3000;
        if (arguments.length === 6) { outputPlace = arg1; shouldReconnect = arg2; reconnectTryTimeout = arg3; }
        else { shouldReconnect = arg1; reconnectTryTimeout = arg2; }
        let val = htmlEventListener + WebForms.GS + path + WebForms.GS + (shouldReconnect ? "1" : "0") + WebForms.GS + reconnectTryTimeout;
        if (outputPlace !== null) val += WebForms.GS + outputPlace;
        this._add("EE" + inputPlace, val);
    }
    
    setFrontEvent(inputPlace, htmlEvent, modulePath, args = null, outputPlace = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("Ej" + inputPlace, htmlEvent + WebForms.GS + modulePath + WebForms.GS + outputPlace + argsJoin);
    }
    
    setFrontEventListener(inputPlace, htmlEventListener, modulePath, args = null, outputPlace = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("EJ" + inputPlace, htmlEventListener + WebForms.GS + modulePath + WebForms.GS + outputPlace + argsJoin);
    }
    
    setMasterPagesEvent(inputPlace, htmlEvent, outputPlace = null) { this._add("Eu" + inputPlace, htmlEvent + WebForms.GS + outputPlace); }
    setMasterPagesEventListener(inputPlace, htmlEventListener, outputPlace = null) { this._add("EU" + inputPlace, htmlEventListener + WebForms.GS + outputPlace); }
    setPreventDefaultEvent(inputPlace, htmlEvent) { this._add("Ed" + inputPlace, htmlEvent); }
    setPreventDefaultEventListener(inputPlace, htmlEventListener) { this._add("ED" + inputPlace, htmlEventListener); }
    setStopPropagationEvent(inputPlace, htmlEvent) { this._add("Es" + inputPlace, htmlEvent); }
    setStopPropagationEventListener(inputPlace, htmlEventListener) { this._add("ES" + inputPlace, htmlEventListener); }
    
    setMethodEvent(inputPlace, htmlEvent, methodName, args = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("Em" + inputPlace, htmlEvent + WebForms.GS + methodName + argsJoin);
    }
    
    setMethodEventListener(inputPlace, htmlEventListener, methodName, args = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("EM" + inputPlace, htmlEventListener + WebForms.GS + methodName + argsJoin);
    }
    
    setModuleMethodEvent(inputPlace, htmlEvent, methodName, args = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("Ex" + inputPlace, htmlEvent + WebForms.GS + methodName + argsJoin);
    }
    
    setModuleMethodEventListener(inputPlace, htmlEventListener, methodName, args = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("EX" + inputPlace, htmlEventListener + WebForms.GS + methodName + argsJoin);
    }
    
    assignConfirmEvent(inputPlace, htmlEvent, text = "Are you sure you want to proceed?", type = "none", title = "Confirm", okText = "OK", cancelText = "Cancel") {
        this._add("Ef" + inputPlace, htmlEvent + WebForms.GS + (text === "Are you sure you want to proceed?" ? "" : text) + WebForms.GS + (type === "none" ? "" : type) + WebForms.GS + (title === "Confirm" ? "" : title) + WebForms.GS + (okText === "OK" ? "" : okText) + WebForms.GS + (cancelText === "Cancel" ? "" : cancelText));
    }
    
    removePostEvent(inputPlace, htmlEvent) { this._add("Rp" + inputPlace, htmlEvent); }
    removePostEventListener(inputPlace, htmlEventListener) { this._add("RP" + inputPlace, htmlEventListener); }
    removeGetEvent(inputPlace, htmlEvent) { this._add("Rg" + inputPlace, htmlEvent); }
    removeGetEventListener(inputPlace, htmlEventListener) { this._add("RG" + inputPlace, htmlEventListener); }
    removePutEvent(inputPlace, htmlEvent) { this._add("Rt" + inputPlace, htmlEvent); }
    removePutEventListener(inputPlace, htmlEventListener) { this._add("RT" + inputPlace, htmlEventListener); }
    removePatchEvent(inputPlace, htmlEvent) { this._add("Ra" + inputPlace, htmlEvent); }
    removePatchEventListener(inputPlace, htmlEventListener) { this._add("RA" + inputPlace, htmlEventListener); }
    removeDeleteEvent(inputPlace, htmlEvent) { this._add("Rl" + inputPlace, htmlEvent); }
    removeDeleteEventListener(inputPlace, htmlEventListener) { this._add("RL" + inputPlace, htmlEventListener); }
    removeOptionsEvent(inputPlace, htmlEvent) { this._add("Ro" + inputPlace, htmlEvent); }
    removeOptionsEventListener(inputPlace, htmlEventListener) { this._add("RO" + inputPlace, htmlEventListener); }
    removeHeadEvent(inputPlace, htmlEvent) { this._add("Rh" + inputPlace, htmlEvent); }
    removeHeadEventListener(inputPlace, htmlEventListener) { this._add("RH" + inputPlace, htmlEventListener); }
    removeSendEvent(inputPlace, htmlEvent) { this._add("Rn" + inputPlace, htmlEvent); }
    removeSendEventListener(inputPlace, htmlEventListener) { this._add("RN" + inputPlace, htmlEventListener); }
    removeCommentEvent(inputPlace, htmlEvent) { this._add("Rb" + inputPlace, htmlEvent); }
    removeCommentEventListener(inputPlace, htmlEventListener) { this._add("RB" + inputPlace, htmlEventListener); }
    removeWasmEvent(inputPlace, htmlEvent) { this._add("Ry" + inputPlace, htmlEvent); }
    removeWasmEventListener(inputPlace, htmlEventListener) { this._add("RY" + inputPlace, htmlEventListener); }
    removeWebSocketEvent(inputPlace, htmlEvent) { this._add("Rw" + inputPlace, htmlEvent); }
    removeWebSocketEventListener(inputPlace, htmlEventListener) { this._add("RW" + inputPlace, htmlEventListener); }
    removeSSEEvent(inputPlace, htmlEvent) { this._add("Re" + inputPlace, htmlEvent); }
    removeSSEEventListener(inputPlace, htmlEventListener) { this._add("RE" + inputPlace, htmlEventListener); }
    removeFrontEvent(inputPlace, htmlEvent) { this._add("Rj" + inputPlace, htmlEvent); }
    removeFrontEventListener(inputPlace, htmlEventListener) { this._add("RJ" + inputPlace, htmlEventListener); }
    removePreventDefaultEvent(inputPlace, htmlEvent) { this._add("Rd" + inputPlace, htmlEvent); }
    removePreventDefaultEventListener(inputPlace, htmlEventListener) { this._add("RD" + inputPlace, htmlEventListener); }
    removeMasterPagesEvent(inputPlace, htmlEvent) { this._add("Ru" + inputPlace, htmlEvent); }
    removeMasterPagesEventListener(inputPlace, htmlEventListener) { this._add("RU" + inputPlace, htmlEventListener); }
    removeStopPropagationEvent(inputPlace, htmlEvent) { this._add("Rs" + inputPlace, htmlEvent); }
    removeStopPropagationEventListener(inputPlace, htmlEventListener) { this._add("RS" + inputPlace, htmlEventListener); }
    removeMethodEvent(inputPlace, htmlEvent, methodName) { this._add("Rm" + inputPlace, htmlEvent + WebForms.GS + methodName); }
    removeMethodEventListener(inputPlace, htmlEventListener, methodName) { this._add("RM" + inputPlace, htmlEventListener + WebForms.GS + methodName); }
    removeModuleMethodEvent(inputPlace, htmlEvent, methodName) { this._add("Rx" + inputPlace, htmlEvent + WebForms.GS + methodName); }
    removeModuleMethodEventListener(inputPlace, htmlEventListener, methodName) { this._add("RX" + inputPlace, htmlEventListener + WebForms.GS + methodName); }
    removeConfirmEvent(inputPlace, htmlEvent) { this._add("Rf" + inputPlace, htmlEvent); }

    // Custom Event
    // This Method Is Compatible With EventListener And May Not Be Compatible With Events Written As Attributes In Some Browsers.
    // Watch: attribute, style, text, children, value
    // Compare: greater, less, equal, notequal, includes, startswith, endswith, matches, changed, inrange, lengthgreater, lengthless, lengthequal
    // Range: Only Use For Compare With inrange Value. Split By Comma ","
    // Key: Only Use For Watch With attribute And style Value
    createCustomDOMEvent(inputPlace, eventName, watch, key, compare, value, range, arg1 = false, arg2 = "0") {
        let immediate = false, delay = "0";
        if (arguments.length === 9) { immediate = arg1; delay = arg2; }
        else { immediate = arg1; delay = arg2; }
        this._add("eC" + inputPlace, eventName + WebForms.GS + watch + WebForms.GS + key + WebForms.GS + compare + WebForms.GS + value + WebForms.GS + range + WebForms.GS + (immediate ? "1" : "0") + WebForms.GS + delay);
    }
    
    enableScrollBottomEvent(enable = true) { this._add("eb", enable ? "1" : "0"); }
    enableReachedElementEvent(inputPlace, once, enable = true) { this._add("er" + inputPlace, (once ? "1" : "0") + WebForms.GS + (enable ? "1" : "0")); }

    // Module
    loadModule(modulePath, methods = null) {
        if (methods === null) methods = [];
        this._add("Ml", modulePath + (methods.length > 0 ? WebForms.GS + "[" + methods.join(WebForms.US) : ""));
    }
    unloadModule(modulePath) { this._add("Mu", modulePath); }
    deleteModuleMethod(methodName) { this._add("Md", methodName); }

    // Unit Testing
    // InputPlace Is Actual, Expected Is Tag/OutputPlace
    assertEqual(inputPlace, tag) { this._add("At" + inputPlace, tag.replace(/\n/g, "$[ln];")); }
    assertEqualByOutputPlace(inputPlace, outputPlace) { this._add("Ao" + inputPlace, outputPlace); }

    // Debug
    createDebugger(pause = false) { this._add("Dc", pause ? "1" : "0"); }
	try() { this._add("tr"); }
	catch() { this._add("ca"); }
	comment(text) { this._add("//", text); }

    // Service Worker
    // To Use Service Worker, You Need To Add The Elanat Dedicated Module (service-worker.js) On The Client Side
    serviceWorkerRegister(path = null, scopePath = null) { this._add("wR", (path !== null ? path : "") + WebForms.GS + (scopePath !== null ? scopePath : "")); }
    serviceWorkerPreCacheStatic(pathList) { this._add("wp", pathList.join(WebForms.GS)); }
    
    serviceWorkerDynamicCache(path, arg1 = "") {
        let seconds = "";
        if (typeof arg1 === "number") seconds = arg1 > 0 ? arg1 : "";
        else seconds = arg1;
        this._add("wc", path + (seconds !== "" ? WebForms.GS + seconds : ""));
    }
    
    serviceWorkerDeleteDynamicCache(path = null) {
        if (path !== null) this._add("wd", path);
        else this._add("wd");
    }
    
    serviceWorkerDynamicCacheTTLUpdate(path, arg1 = "") {
        let seconds = "";
        if (typeof arg1 === "number") seconds = arg1 > 0 ? arg1 : "";
        else seconds = arg1;
        this._add("wt", path + (seconds !== "" ? WebForms.GS + seconds : ""));
    }
    
    // Path: Support Wildcard Automatically And Also Support Regex If Use "re:" Before Pattern
    // Type: Type Is Cache Strategy. cachefirst, networkfirst, cacheonly, networkonly, stalerevalidate (Fast From Cache, Updates Simultaneously From The Network)
    // CacheDynamic: If True, Any Successful Network Response For That Route Will Be Stored In The Dynamic Cache
    serviceWorkerRouteSet(path, type, cacheDynamic = false) { this._add("wr", path + WebForms.GS + type + (cacheDynamic ? WebForms.GS + "1" : "")); }
    serviceWorkerRouteAlias(path, to) { this._add("wa", path + WebForms.GS + to); }
    
    serviceWorkerDeleteRouteAlias(path = null) {
        if (path !== null) this._add("wC", path);
        else this._add("wC");
    }
    
    // Delete All Route And Alias
    serviceWorkerDeleteRoute(path = null) {
        if (path !== null) this._add("wD", path);
        else this._add("wD");
    }

    // SSE
    disconnectSSE(path = null) {
        if (path !== null) this._add("Ds", path);
        else this._add("Ds");
    }
    disconnectAllSSE() { this._add("Ds"); }

    // State
    addState(path = null, title = null) { this._add("AS", (path !== null ? path : "") + WebForms.GS + (title !== null ? title : "")); }
    saveState(path = null, title = null) { this._add("As", (path !== null ? path : "") + WebForms.GS + (title !== null ? title : "")); }
    loadState(path) { this._add("ls", path); }
    
    deleteState(path = null) {
        if (path !== null) this._add("DS", path);
        else this._add("DS", "*");
    }
    deleteAllState() { this._add("DS", "*"); }
	lockQueue(millisecond) { this._add("lq", typeof millisecond === "number" ? millisecond.toString() : millisecond); }

    // Cookie
    setCookie(key, value, seconds, path = null) {
        this._add("sC", key + WebForms.GS + value + WebForms.GS + seconds + (path !== null ? WebForms.GS + path : ""));
    }

    // Save (Session Cache)
    saveId(inputPlace, key = ".") { this._add("@gi" + inputPlace, key); }
    saveName(inputPlace, key = ".") { this._add("@gn" + inputPlace, key); }
    saveValue(inputPlace, key = ".") { this._add("@gv" + inputPlace, key); }
    saveValueLength(inputPlace, key = ".") { this._add("@ge" + inputPlace, key); }
    saveClass(inputPlace, key = ".") { this._add("@gc" + inputPlace, key); }
    saveStyle(inputPlace, key = ".") { this._add("@gs" + inputPlace, key); }
    saveTitle(inputPlace, key = ".") { this._add("@gl" + inputPlace, key); }
    saveLabel(inputPlace, key = ".") { this._add("@gA" + inputPlace, key); }
    saveText(inputPlace, key = ".") { this._add("@gt" + inputPlace, key); }
    saveOuterText(inputPlace, key = ".") { this._add("@go" + inputPlace, key); }
    saveTextLength(inputPlace, key = ".") { this._add("@gg" + inputPlace, key); }
    saveAttribute(inputPlace, attribute, key = ".") { this._add("@ga" + inputPlace, key + WebForms.GS + attribute); }
    saveWidth(inputPlace, key = ".") { this._add("@gw" + inputPlace, key); }
    saveHeight(inputPlace, key = ".") { this._add("@gh" + inputPlace, key); }
    saveReadOnly(inputPlace, key = ".") { this._add("@gr" + inputPlace, key); }
    saveSelectedIndex(inputPlace, key = ".") { this._add("@gx" + inputPlace, key); }
    saveTextAlign(inputPlace, key = ".") { this._add("@gT" + inputPlace, key); }
    saveNodeLength(inputPlace, key = ".") { this._add("@gL" + inputPlace, key); }
    saveVisible(inputPlace, key = ".") { this._add("@gV" + inputPlace, key); }
    saveUrl(url, fetchScript = false, key = ".") { this._add("@gu", key + WebForms.GS + url + (fetchScript ? WebForms.GS + "1" : "")); }
    saveIndex(inputPlace, key = ".") { this._add("@gI" + inputPlace, key); }
    removeSave(cacheKey) { this._add("rs", cacheKey); }
    removeAllSave() { this._add("rs", "*"); }
    
    // Calling the SetSave Method Causes Action Control Requests Triggered by Events Using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send Event, to be Temporarily Saved on the Active Page, so the Request will not be Sent to the Server Again.
    setSave() { this._add("cs", "*"); }
    addSaveValue(cacheKey, value) { this._add("SA", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];")); }
    insertSaveValue(cacheKey, value) { this._add("SI", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];")); }
    appendSaveValue(cacheKey, value) { this._add("SP", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];")); }
    replaceSaveValue(cacheKey, searchValue, value) { this._add("SR", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];") + WebForms.GS + searchValue.replace(/\n/g, "$[ln];")); }
	// Is Regex Replace
	setFormatSaveValue(cacheKey, regex, replacement) {
		this._add("SF", cacheKey + WebForms.GS + regex + WebForms.GS + replacement.replace(/\n/g, "$[ln];"));
	}
	// Operator: +, -, *, /, %, //, **
	setArithmeticSaveValue(cacheKey, operator, value) {
		this._add("SM", cacheKey + WebForms.GS + operator + WebForms.GS + (typeof value === "number" ? value.toString() : value));
	}
	// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
	setTextOperationSaveValue(cacheKey, operation, value1, value2) {
		this._add(
			"ST",
			cacheKey + WebForms.GS + operation + WebForms.GS +
			(typeof value1 === "number" ? value1.toString() : value1) + WebForms.GS +
			(typeof value2 === "number" ? value2.toString() : value2)
		);
	}

    // Cache
    cacheId(inputPlace, key = ".") { this._add("@ci" + inputPlace, key); }
    cacheName(inputPlace, key = ".") { this._add("@cn" + inputPlace, key); }
    cacheValue(inputPlace, key = ".") { this._add("@cv" + inputPlace, key); }
    cacheValueLength(inputPlace, key = ".") { this._add("@ce" + inputPlace, key); }
    cacheClass(inputPlace, key = ".") { this._add("@cc" + inputPlace, key); }
    cacheStyle(inputPlace, key = ".") { this._add("@cs" + inputPlace, key); }
    cacheTitle(inputPlace, key = ".") { this._add("@cl" + inputPlace, key); }
    cacheLabel(inputPlace, key = ".") { this._add("@cA" + inputPlace, key); }
    cacheText(inputPlace, key = ".") { this._add("@ct" + inputPlace, key); }
    cacheOuterText(inputPlace, key = ".") { this._add("@co" + inputPlace, key); }
    cacheTextLength(inputPlace, key = ".") { this._add("@cg" + inputPlace, key); }
    cacheAttribute(inputPlace, attribute, key = ".") { this._add("@ca" + inputPlace, key + WebForms.GS + attribute); }
    cacheWidth(inputPlace, key = ".") { this._add("@cw" + inputPlace, key); }
    cacheHeight(inputPlace, key = ".") { this._add("@ch" + inputPlace, key); }
    cacheReadOnly(inputPlace, key = ".") { this._add("@cr" + inputPlace, key); }
    cacheSelectedIndex(inputPlace, key = ".") { this._add("@cx" + inputPlace, key); }
    cacheTextAlign(inputPlace, key = ".") { this._add("@cT" + inputPlace, key); }
    cacheNodeLength(inputPlace, key = ".") { this._add("@cL" + inputPlace, key); }
    cacheVisible(inputPlace, key = ".") { this._add("@cV" + inputPlace, key); }
    cacheUrl(url, fetchScript = false, key = ".") { this._add("@cu", key + WebForms.GS + url + (fetchScript ? WebForms.GS + "1" : "")); }
    cacheIndex(inputPlace, key = ".") { this._add("@cI" + inputPlace, key); }
    removeCache(cacheKey) { this._add("rd", cacheKey); }
    removeAllCache() { this._add("rd", "*"); }
    
    // Calling the SetCache Method Causes Action Control Requests Triggered by events using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send event, to be Cached, so the Request will not be Sent to the Server Again.
    setCache(arg1 = null) {
        if (arg1 === null) this._add("cd", "*");
        else this._add("cd", arg1);
    }
    addCacheValue(cacheKey, value) { this._add("CA", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];")); }
    insertCacheValue(cacheKey, value) { this._add("CI", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];")); }
    appendCacheValue(cacheKey, value) { this._add("CP", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];")); }
    replaceCacheValue(cacheKey, searchValue, value) { this._add("CR", cacheKey + WebForms.GS + value.replace(/\n/g, "$[ln];") + WebForms.GS + searchValue.replace(/\n/g, "$[ln];")); }
	// Is Regex Replace
	setFormatCacheValue(cacheKey, regex, replacement) {
		this._add(
			"CF",
			cacheKey + WebForms.GS + regex + WebForms.GS +
			replacement.replace(/\n/g, "$[ln];")
		);
	}
	// Operator: +, -, *, /, %, //, **
	setArithmeticCacheValue(cacheKey, operator, value) {
		this._add(
			"CM",
			cacheKey + WebForms.GS + operator + WebForms.GS +
			(typeof value === "number" ? value.toString() : value)
		);
	}
	// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
	setTextOperationCacheValue(cacheKey, operation, value1, value2) {
		this._add(
			"CT",
			cacheKey + WebForms.GS + operation + WebForms.GS +
			(typeof value1 === "number" ? value1.toString() : value1) + WebForms.GS +
			(typeof value2 === "number" ? value2.toString() : value2)
		);
	}

    // Call
    loadUrl(inputPlace, url) { this._add("lu" + inputPlace, url); }
    runActionControls(actionControls, withoutWebFormsSection = true, index = null, useCurrentEvent = true) { this._add("lA", (useCurrentEvent ? "1" : "0") + WebForms.GS + (withoutWebFormsSection ? "1" : "0") + WebForms.GS + index + WebForms.GS + actionControls); }
    callScript(scriptText) { this._add("_", scriptText.replace(/\n/g, "$[ln];")); }
    
    callMethod(methodName, args = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("lm", methodName + argsJoin);
    }
    
    callModuleMethod(methodName, args = null) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("lM", methodName + argsJoin);
    }
    
    callPostBack(formInputPlace, outputPlace = null) { this._add("Lp", "1" + WebForms.GS + formInputPlace + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }
    
    callCommentBack(arg1 = null, arg2 = null, arg3 = true) {
        let index = null, inputPlace = null, useCurrentEvent = true;
        if (arguments.length === 1) { index = arg1; }
        else if (arguments.length === 2) { index = arg1; inputPlace = arg2; }
        else { index = arg1; inputPlace = arg2; useCurrentEvent = arg3; }
        this._add("LC", (useCurrentEvent ? "1" : "0") + WebForms.GS + (index !== null ? index : "") + WebForms.GS + inputPlace);
    }
    
    callWasmBack(wasmLanguage, wasmUrl, methodName, args = null, outputPlace = null, useCurrentEvent = true) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = "[" + args.map(String).join(WebForms.US);
        this._add("Ly", (useCurrentEvent ? "1" : "0") + WebForms.GS + wasmLanguage + WebForms.GS + wasmUrl + WebForms.GS + methodName + WebForms.GS + argsJoin + WebForms.GS + outputPlace);
    }
    
    callWebSocketBack(path, useCurrentEvent = true) { this._add("Lw", (useCurrentEvent ? "1" : "0") + WebForms.GS + path); }
    
    callSSEBack(path, arg1 = null, arg2 = true, arg3 = true, arg4 = "3000") {
        let outputPlace = null, useCurrentEvent = true, shouldReconnect = true, reconnectTryTimeout = "3000";
        if (arguments.length === 5) { outputPlace = arg1; useCurrentEvent = arg2; shouldReconnect = arg3; reconnectTryTimeout = arg4; }
        else { useCurrentEvent = arg1; shouldReconnect = arg2; reconnectTryTimeout = arg3; }
        let val = (useCurrentEvent ? "1" : "0") + WebForms.GS + path + WebForms.GS + (shouldReconnect ? "1" : "0") + WebForms.GS + reconnectTryTimeout;
        if (outputPlace !== null) val += WebForms.GS + outputPlace;
        this._add("Ls", val);
    }
    
    callFront(modulePath, args = null, outputPlace = null, useCurrentEvent = true) {
        let argsJoin = "";
        if (args !== null && args.length > 0) argsJoin = WebForms.GS + "[" + args.map(String).join(WebForms.US);
        this._add("Lj", (useCurrentEvent ? "1" : "0") + WebForms.GS + modulePath + WebForms.GS + outputPlace + argsJoin);
    }
    
    callGetBack(path, outputPlace = null, useCurrentEvent = true) { this._add("Lg", (useCurrentEvent ? "1" : "0") + WebForms.GS + path + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }
    callPutBack(path, outputPlace = null, useCurrentEvent = true) { this._add("Lt", (useCurrentEvent ? "1" : "0") + WebForms.GS + path + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }
    callPatchBack(path, outputPlace = null, useCurrentEvent = true) { this._add("LP", (useCurrentEvent ? "1" : "0") + WebForms.GS + path + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }
    callDeleteBack(path, outputPlace = null, useCurrentEvent = true) { this._add("Ld", (useCurrentEvent ? "1" : "0") + WebForms.GS + path + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }
    callHeadBack(path, useCurrentEvent = true) { this._add("Lh", (useCurrentEvent ? "1" : "0") + WebForms.GS + path); }
    callOptionsBack(path, outputPlace = null, useCurrentEvent = true) { this._add("Lo", (useCurrentEvent ? "1" : "0") + WebForms.GS + path + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }
    callSendBack(path, method, isMultiPart, contentType, data, outputPlace = null, useCurrentEvent = true) { this._add("LS", (useCurrentEvent ? "1" : "0") + WebForms.GS + path + WebForms.GS + method + WebForms.GS + (isMultiPart ? "1" : "0") + WebForms.GS + contentType + WebForms.GS + data.replace(/\n/g, "$[ln];") + (outputPlace !== null ? WebForms.GS + outputPlace : "")); }

    // Update
    increase(inputPlace, value) { this._add("gt" + inputPlace, "i" + WebForms.GS + value); }
    decrease(inputPlace, value) { this._add("gt" + inputPlace, "i" + WebForms.GS + (value * -1)); }
    
    // If You Don't Use Deep Mode, any Tags Inside the Current Tag Will Simply Be Treated as Strings. Deep Mode Does not Remove Inner Elements.
    replace(inputPlace, value, newValue, alsoStartTag = false, deep = true) { this._add("gt" + inputPlace, "r" + WebForms.GS + value + WebForms.GS + newValue + WebForms.GS + (alsoStartTag ? "1" : "0") + WebForms.GS + (deep ? "1" : "0")); }
    
    // HTML Converts Attribute Names to Lowercase, so they Need to Be Written in Lowercase.
    replaceStartTag(inputPlace, value, newValue) { this._add("gt" + inputPlace, "s" + WebForms.GS + value + WebForms.GS + newValue); }

    // Pre Runner
    assignDelay(milliSecond, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        const newName = ":" + milliSecond + ")" + parts[0];
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }
    
    assignDelayChange(milliSecond, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        let currentName = parts[0];
        if (currentName.startsWith(":") && currentName.includes(")")) {
            const closingBracket = currentName.indexOf(")");
            currentName = currentName.substring(closingBracket + 1);
        }
        const newName = ":" + milliSecond + ")" + currentName;
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }
    
    assignInterval(milliSecond, id = null, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        const newName = "(" + milliSecond + (id !== null ? "|" + id : "") + ")" + parts[0];
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }
    
    assignIntervalChange(milliSecond, id = null, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        let currentName = parts[0];
        if (currentName.startsWith("(") && currentName.includes(")")) {
            const closingBracket = currentName.indexOf(")");
            currentName = currentName.substring(closingBracket + 1);
        }
        const newName = "(" + milliSecond + (id !== null ? "|" + id : "") + ")" + currentName;
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }
    
    deleteInterval(id) { this._add("Di", id); }
    
    assignRepeat(count, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        const newName = "," + count + ")" + parts[0];
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }
    
    assignRepeatChange(count, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        let currentName = parts[0];
        if (currentName.startsWith(",") && currentName.includes(")")) {
            const closingBracket = currentName.indexOf(")");
            currentName = currentName.substring(closingBracket + 1);
        }
        const newName = "," + count + ")" + currentName;
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }

    // Index
    startIndex(name = null) { this._add("#", name !== null ? name : ""); }
    
    // This Index Is Automatically Run After Changing The Browser History (Back And Forward Buttons)
    startState() { this.startIndex("$"); }
    
    goTo(line, repeat = 1) { this._add("&", line + WebForms.GS + repeat); }
    goToIndex(index, repeat = 1) { this._add("&", "#" + index + WebForms.GS + repeat); }

    // Start
    startTransientDOM(inputPlace) { this._add("td", inputPlace); }
    endTransientDOM() { this._add("td", ";"); }

    // Message
    // Type: warning, problem, help, success, none
    alert(text, type = "none", title = "Alert", okText = "OK") { this._add("Al", text + WebForms.GS + (type === "none" ? "" : type) + WebForms.GS + (title === "Alert" ? "" : title) + WebForms.GS + (okText === "OK" ? "" : okText)); }
    
    message(text, arg1 = "none", arg2 = "0") {
        if (typeof arg1 === "number") {
            this._add("me", text + WebForms.GS + "" + WebForms.GS + arg1);
        } else if (arguments.length === 3) {
            this._add("me", text + WebForms.GS + arg1 + WebForms.GS + arg2);
        } else {
            this._add("me", text + WebForms.GS + (arg1 === "none" ? "" : arg1) + WebForms.GS + "");
        }
    }
    
    // Type: log, info, warn, error, debug, trace, group, groupend, table
    consoleMessage(text, type = "log") { this._add("mc", text.replace(/\n/g, "$[ln];") + (type === "log" ? "" : WebForms.GS + type)); }
    consoleMessageAssert(text, condition) { this._add("ma", text.replace(/\n/g, "$[ln];") + WebForms.GS + condition); }

    // Enable
    //Calling The EnableWebSocket Or EnableWebSocketOnce Or AddWebSocket Methods Will Cause Any Subsequent Requests (Under WebForms Core Technology) To Operate Under The WebSocket Protocol.
    enableWebSocket(enable = true) { this._add("ew", enable ? "1" : "0"); }
    enableWebSocketOnce() { this._add("ew", "$"); }
    addWebSocket(path) { this._add("aw" + path); }
    
    // Disconnected WebSocket
    deleteWebSocket(path) { this._add("dw" + path); }

    // Use
    // InputPlace Using Only For form Element
    useWebSocket(inputPlace) { this._add("uw" + inputPlace); }
    useOnlyChangeUpdate(inputPlace) { this._add("uo" + inputPlace); }

    // Condition And Loop
    // Condition And Loop Supports Brackets and Then
    // Type: warning, problem, help, success, none
    // Interval: Value 0 is Await (if is not True, all Next Action Controls Waiting for it), Value -1 is Sync Check Once (is Support Bracket or Next Action Control), Value > 0 is Async and is Wait Based on Time Repetition Until it Becomes True (Is Support Bracket or Next Action Control, but is not Support Else).
    // Nested Conditions and Nested Loops are Possible.
    _condPrefix(interval) { return (interval >= 0 ? "{(" + interval + ")" : "{"); }
    
    confirmIsTrueAccept(text = "Are you sure you want to proceed?", type = "none", title = "Confirm", okText = "OK", cancelText = "Cancel", interval = 100) {
        this._add(this._condPrefix(interval) + "ct", (text === "Are you sure you want to proceed?" ? "" : text) + WebForms.GS + (type === "none" ? "" : type) + WebForms.GS + (title === "Confirm" ? "" : title) + WebForms.GS + (okText === "OK" ? "" : okText) + WebForms.GS + (cancelText === "Cancel" ? "" : cancelText));
        return this;
    }
    
    confirmIsFalseAccept(text = "Are you sure you want to proceed?", type = "none", title = "Confirm", okText = "OK", cancelText = "Cancel", interval = 100) {
        this._add(this._condPrefix(interval) + "cf", (text === "Are you sure you want to proceed?" ? "" : text) + WebForms.GS + (type === "none" ? "" : type) + WebForms.GS + (title === "Confirm" ? "" : title) + WebForms.GS + (okText === "OK" ? "" : okText) + WebForms.GS + (cancelText === "Cancel" ? "" : cancelText));
        return this;
    }
    
    isGreaterThan(firstValue, secondValue, interval = -1) { this._add(this._condPrefix(interval) + "gt", firstValue + WebForms.GS + secondValue); return this; }
    isLessThan(firstValue, secondValue, interval = -1) { this._add(this._condPrefix(interval) + "lt", firstValue + WebForms.GS + secondValue); return this; }
    isEqualTo(firstValue, secondValue, interval = -1) { this._add(this._condPrefix(interval) + "et", firstValue + WebForms.GS + secondValue); return this; }
    isNotEqualTo(firstValue, secondValue, interval = -1) { this._add(this._condPrefix(interval) + "Nt", firstValue + WebForms.GS + secondValue); return this; }
    exist(value, interval = -1) { this._add(this._condPrefix(interval) + "ex", value); return this; }
    notExist(value, interval = -1) { this._add(this._condPrefix(interval) + "nx", value); return this; }
    isTrue(value, interval = -1) { this._add(this._condPrefix(interval) + "tr", value); return this; }
    isFalse(value, interval = -1) { this._add(this._condPrefix(interval) + "fa", value); return this; }
    isMatchMedia(value, interval = -1) { this._add(this._condPrefix(interval) + "mm", value); return this; }
    isNotMatchMedia(value, interval = -1) { this._add(this._condPrefix(interval) + "nm", value); return this; }
    include(text, value, interval = -1) { this._add(this._condPrefix(interval) + "In", value + WebForms.GS + text); return this; }
    notInclude(text, value, interval = -1) { this._add(this._condPrefix(interval) + "Nn", value + WebForms.GS + text); return this; }
    elementExists(inputPlace, interval = -1) { this._add(this._condPrefix(interval) + "eE", inputPlace); return this; }
    elementNotExists(inputPlace, interval = -1) { this._add(this._condPrefix(interval) + "nE", inputPlace); return this; }
    isRegexMatch(value, pattern, interval = -1) { this._add(this._condPrefix(interval) + "re", value + WebForms.GS + pattern); return this; }
    isRegexNotMatch(value, pattern, interval = -1) { this._add(this._condPrefix(interval) + "rn", value + WebForms.GS + pattern); return this; }
    
    // In: Everything Becomes A JSON List.
    // Key: Creates A Temporary Data In The Browser IndexedDB.
    // Key + "i" Creates A Temporary Data To Maintain The Loop Counter In The Browser IndexedDB.
    forEach(path, inVal, key = ".") { this._add("{fe", path + WebForms.GS + inVal + WebForms.GS + key); return this; }
    
    break() { this._add(";"); }
    else() { this._add("}e"); return this; }
    startBracket() { this._add("{"); }
    endBracket() { this._add("}"); }

	// High-Level Methods
    // Used Then In Condition And Loop Methods
    then(newForm) {
        if (newForm === null) return this;
        const data = newForm.getWebFormsData();
        if (data !== "") {
            if (data.includes("\n")) {
                newForm._addToUp("{");
                newForm._add("}");
            }
        }
        this.appendForm(newForm);
        return this;
    }
    
    thenClosure(configure) {
        const newForm = new WebForms();
        configure(newForm);
        const data = newForm.getWebFormsData();
        if (data !== "") {
            if (data.includes("\n")) {
                newForm._addToUp("{");
                newForm._add("}");
            }
        }
        this.appendForm(newForm);
        return this;
    }
    
    repeat(newForm, repeat) {
        if (newForm === null) return this;
        const bodyData = newForm.getWebFormsData();
        if (bodyData === "") return this;
        const startLine = bodyData.split("\n").length * -1;
        this.appendForm(newForm);
        this.goTo(startLine, repeat - 1);
        return this;
    }
    
    repeatWithIndex(newForm, repeat, index) {
        if (newForm === null) return this;
        this.goToIndex(index);
        this.startIndex(index);
        const bodyData = newForm.getWebFormsData();
        if (bodyData === "") return this;
        this.appendForm(newForm);
        if (index === "") {
            let indexNumber = -1;
            for (const x of this.getWebFormsData().split("\n")) {
                if (x.startsWith("#")) indexNumber++;
            }
            this.goTo(indexNumber, repeat - 1);
        } else {
            this.goToIndex(index, repeat - 1);
        }
        return this;
    }
    
    repeatClosure(configure, repeat) {
        const newForm = new WebForms();
        configure(newForm);
        return this.repeat(newForm, repeat);
    }
    
    repeatClosureWithIndex(configure, repeat, index) {
        const newForm = new WebForms();
        configure(newForm);
        return this.repeatWithIndex(newForm, repeat, index);
    }

	isole(newForm, inputPlace) {
		if (newForm === null) return this;

		const bodyData = newForm.getWebFormsData();
		if (bodyData === "") return this;

		const form = new WebForms();
		form.startTransientDOM(inputPlace);

		this.appendForm(form);
		newForm.endTransientDOM();
		this.appendForm(newForm);

		return this;
	}

	isoleClosure(configure, inputPlace) {
		const newForm = new WebForms();
		configure(newForm);
		return this.isole(newForm, inputPlace);
	}

	// The Render Method is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
	// This Method Utilizes the Transient DOM; Therefore, When Selecting InputPlaces, You Must Consider the Root of the Selected InputPlace Within the Method.
	render(newForm, inputPlace, key = "", permanent = false) {
		if (newForm === null) return this;

		const bodyData = newForm.getWebFormsData();
		if (bodyData === "") return this;

		if (key === "") key = inputPlace;

		const form = new WebForms();

		form.exist(permanent ? Fetch.cache(key) : Fetch.save(key));
			form.rollback(inputPlace, key, permanent);
		form.else();
			form.snapshot(inputPlace, key, permanent);

		form.startTransientDOM(inputPlace);

		this.appendForm(form);
		newForm.endTransientDOM();
		this.appendForm(newForm);

		return this;
	}

	renderClosure(configure, inputPlace, key = "", permanent = false) {
		const newForm = new WebForms();
		configure(newForm);
		return this.render(newForm, inputPlace, key, permanent);
	}

    // Async
    // It Supports Brackets and Then
    async() { this._add("{(a)"); return this; }
    delay(milliSecond) { this._add("De", milliSecond); }

    // Option
    changeOption(name, value) { this._add("co", name + WebForms.GS + value); }
    resetOption(name = null) {
        if (name !== null) this._add("ro", name);
        else this._add("ro");
    }

    // Format Storage
    createFormatStorage(key, data) { this._add(".C", key + WebForms.GS + data); }
    deleteFormatStorage(key) { this._add(".D", key); }
    addJSON(key, path, value) { this._add(".a", key + WebForms.GS + "j" + WebForms.GS + value + WebForms.GS + path); }
    
    // Name: For Support Attribute, Set Double At Sign (@@) Before Name.
    addXML(key, path, name, value = null) { this._add(".a", key + WebForms.GS + "x" + WebForms.GS + name + WebForms.GS + (value !== null ? value : "") + WebForms.GS + path); }
    
    addINI(key, path, value, isINILike = false) { this._add(".a", key + WebForms.GS + "i" + WebForms.GS + (isINILike ? "1" : "0") + WebForms.GS + value + WebForms.GS + path); }
    addTextLine(key, line, text) { this._add(".a", key + WebForms.GS + "t" + WebForms.GS + text + WebForms.GS + line); }
    addVariable(key, value) { this._add(".a", key + WebForms.GS + "v" + WebForms.GS + value); }
    updateJSON(key, path, value) { this._add(".u", key + WebForms.GS + "j" + WebForms.GS + value + WebForms.GS + path); }
    updateXML(key, path, value) { this._add(".u", key + WebForms.GS + "x" + WebForms.GS + value + WebForms.GS + path); }
    updateINI(key, path, value, isINILike = false) { this._add(".u", key + WebForms.GS + "i" + WebForms.GS + (isINILike ? "1" : "0") + WebForms.GS + value + WebForms.GS + path); }
    updateTexLine(key, line, text) { this._add(".u", key + WebForms.GS + "t" + WebForms.GS + text + WebForms.GS + line); }
    updateVariable(key, value) { this._add(".u", key + WebForms.GS + "v" + WebForms.GS + value); }
    increaseVariable(key, value) { this._add(".i", key + WebForms.GS + "v" + WebForms.GS + value); }
    decreaseVariable(key, value) { this.increaseVariable(key, value * -1); }
    deleteJSON(key, path) { this._add(".d", key + WebForms.GS + "j" + WebForms.GS + path); }
    deleteXML(key, path) { this._add(".d", key + WebForms.GS + "x" + WebForms.GS + path); }
    deleteINI(key, path, isINILike = false) { this._add(".d", key + WebForms.GS + "i" + WebForms.GS + (isINILike ? "1" : "0") + WebForms.GS + path); }
    deleteTextLine(key, line) { this._add(".d", key + WebForms.GS + "t" + WebForms.GS + line); }
    deleteVariable(key) { this._add(".d", key + WebForms.GS + "v"); }

    // Template Engine
    // Pattern Example: {{value}}, ((value)), *value*, $value;
    bindJSONToTemplate(inputPlace, jsonText, path, pattern, alsoStartTag = true) { this._add("Tj" + inputPlace, jsonText + WebForms.GS + path + WebForms.GS + pattern + WebForms.GS + (alsoStartTag ? "1" : "0")); }
    
    // Because XML Elements Are Lowercased, Placeholders Must Use Lowercase Names.
    bindXMLToTemplate(inputPlace, xmlText, path, pattern, alsoStartTag = true) { this._add("Tx" + inputPlace, xmlText + WebForms.GS + path + WebForms.GS + pattern + WebForms.GS + (alsoStartTag ? "1" : "0")); }
    
    bindINIToTemplate(inputPlace, iniText, path, pattern, alsoStartTag = true) { this._add("Ti" + inputPlace, iniText + WebForms.GS + path + WebForms.GS + pattern + WebForms.GS + (alsoStartTag ? "1" : "0")); }

    // Inject
    // Need Add @: to First of String
    inject(value) { return "$[" + value + "];"; }

    // Action Control
    replaceActionControl(searchValue, value, addingToUp = false) {
        if (addingToUp) this._addToUp("rE", searchValue + WebForms.GS + value);
        else this._add("rE", searchValue + WebForms.GS + value);
    }
    
    assignReplace(searchValue, value, index = -1) {
        const currentLine = this._getLineByIndex(index);
        if (currentLine === "") return;
        const parts = currentLine.split("=", 2);
        const newName = ";" + searchValue + WebForms.GS + value + WebForms.GS + parts[0];
        const newValue = parts.length > 1 ? parts[1] : "";
        this._updateLineByIndex(index, newName, newValue);
    }

    // Hash And Checksum
    setHash() { this._add("SH"); }
    setChecksum() { this._add("CS"); }
    
    checksumCalculation(text) {
        let sum = 0;
        const mod = 65536;
        const shift = 5;
        for (let i = 0; i < text.length; i++) {
            const c = text.charCodeAt(i);
            sum = ((sum << shift) | (sum >>> (16 - shift))) ^ c;
            sum %= mod;
        }
        return String(sum);
    }
    
    getChecksum() { return this.checksumCalculation(this.getWebFormsData()); }

    // Get
    getFormsActionData() {
        if (this._webFormsData.length === 0) return "";
        return this._webFormsData;
    }
    
    response() { return "[web-forms]\n" + this.getFormsActionData(); }
    
    getFormsActionDataLineBreak() {
        if (this._webFormsData.length === 0) return "";
        return this._webFormsData.replace(/"/g, "$[dq];").replace(/\n/g, "$[sln];");
    }

    // Export
    exportToHtmlComment(addLine = false) {
        let response = this.response().replace(/--/g, "$[dd];");
        if (response.endsWith("-")) {
            response = response.substring(0, response.length - 1) + "$[da];";
        }
        return (addLine ? "\n" : "") + "<!--" + response + "-->";
    }
    
    // Using it for SSE Response
    exportToLineBreak(src = null) { return "[web-forms]$[sln];" + this.getFormsActionDataLineBreak(); }
    
    getWebFormsData() { return this._webFormsData; }
    
    appendForm(form) {
        if (form === null) return;
        const otherData = form.getWebFormsData();
        if (otherData !== "") {
            if (this._webFormsData.length > 0) this._webFormsData += "\n";
            this._webFormsData += otherData;
        }
    }
    
    clean() { this._webFormsData = ""; }
}

export class Security {
    safeValue(value) {
        if (value.length < 1) return value;
        if (value[0] === "@") value = "@" + value;
        return value.replace(/\n/g, "$[ln];").replace(/,@/g, "$[co];@").replace(/\x1C/g, "").replace(/\x1D/g, "").replace(/\x1E/g, "").replace(/\x1F/g, "");
    }
}

// WebForms Place Criteria (WPC) DSL
export class InputPlace {
    static DOCUMENT = ",";
    static WINDOW = "`";
    // When Calling TransientDOM, Using Root will Result in the Selection of the Transient Tag.
    static ROOT = "~";
    static HTML = ".";
    static HEAD = "^";
    static SCREEN_ORIENTATION = "%";
    static ALL = "*";
    static PARENT = "/";
    static CURRENT = "$";
    static TARGET = "!";
    static UPPER = "-";

    static id(id) { return id; }
    
    static name(name, index = null) { return "(" + name + ")" + (index !== null ? index : ""); }
    static allNames(name) { return "(" + name + ")*"; }
    
    static tag(tag, index = null) { return "<" + tag + ">" + (index !== null ? index : ""); }
    static allTags(tag) { return "<" + tag + ">*"; }
    
    static child(index = null) { return "<>" + (index !== null ? index : ""); }
    static allChild() { return "<>*"; }
    
    static class(className, index = null) { return "{" + className + "}" + (index !== null ? index : ""); }
    static allClasses(className) { return "{" + className + "}*"; }
    
    static attribute(name, arg1 = null, arg2 = null, arg3 = "") {
        if (arguments.length === 1) return '"' + name + '"';
        if (typeof arg1 === "number") return '"' + name + '"' + arg1;
        const op = arg3 !== "" ? arg3 : "";
        if (typeof arg2 === "number") return '"' + name + op + "'" + arg1 + '"' + arg2;
        return '"' + name + op + "'" + arg1 + '"';
    }
    
    static allAttributes(name, value = null, operator = "") {
        if (value === null) return '"' + name + '"*';
        const op = operator !== "" ? operator : "";
        return '"' + name + op + "'" + value + '"*';
    }
    
    static query(query) { return "*" + query.replace(/=/g, "$[eq];").replace(/\|/g, "$[vb];").replace(/\?/g, "$[qu];"); }
    static queryAll(query) { return "[" + query.replace(/=/g, "$[eq];").replace(/\|/g, "$[vb];").replace(/\?/g, "$[qu];"); }
}

export class OutputPlace extends InputPlace {}

// Do not Add any Data Before or After it
export class Fetch {
    static RS = "\x1E";
    static US = "\x1F";

    // Method
    static random(maxValue, minValue = null) {
        return "@mr" + maxValue + (minValue !== null ? Fetch.RS + minValue : "");
    }
    
    static spaceToChar(text, character = "-") { return "@sc" + character + Fetch.RS + text; }
    static encodeURI(text) { return "@ue" + text; }
    static decodeURI(text) { return "@ud" + text; }

    static method(methodName, args = null) {
        return "@cm" + methodName + (args !== null && args.length > 0 ? Fetch.RS + args.map(String).join(Fetch.US) : "");
    }

    static moduleMethod(methodName, args = null) {
        return "@cM" + methodName + (args !== null && args.length > 0 ? Fetch.RS + args.map(String).join(Fetch.US) : "");
    }

    // MethodName: The Method Name May Need to Include the Class Name, Separated by a Period. Example: MyClassName.MyMethodName
    static wasmMethod(wasmLanguage, wasmUrl, methodName, args = null, key = ".") {
        return "@wA" + wasmLanguage + Fetch.RS + wasmUrl + Fetch.RS + methodName + (args !== null && args.length > 0 ? Fetch.RS + args.map(String).join(Fetch.US) : "");
    }

    static script(scriptText) { return "@_" + scriptText.replace(/\n/g, "$[ln];"); }
    static loadUrl(url, fetchScript = false) { return "@lu" + url + (fetchScript ? Fetch.RS + "1" : ""); }
    static loadHtml(url, fetchInputPlace = "", fetchScript = false) { return "@lh" + url + Fetch.RS + (fetchScript ? "1" : "0") + (fetchInputPlace !== "" ? Fetch.RS + fetchInputPlace : ""); }
    static loadLine(url, line) { return "@ll" + url + Fetch.RS + line; }
    static loadINI(url, name, isINILike = false) { return "@li" + url + Fetch.RS + name + (isINILike ? Fetch.RS + "1" : ""); }
    
    // Name: Name Or Nested Paths. Is Supprt Index (Student[8].Name). Nested Paths Index Starts At 0
    static loadJSON(url, name) { return "@lj" + url + Fetch.RS + name; }
    
    // Name: Name Or XPath; XPath Index Starts At 1
    static loadXML(url, name) { return "@lx" + url + Fetch.RS + name; }
    
    // MethodName: It's Check Function Or Variable
    static hasMethod(methodName) { return "@hm" + methodName; }
    static hasModuleMethod(methodName) { return "@hM" + methodName; }
    
    // This Method Return True Or False If Key Pressed
    // Modifier: Alt, AltGraph, Control, Meta, Shift, CapsLock, NumLock, ScrollLock
    static getModifierState(modifier) { return "@ms" + modifier; }

    // Math
    static math(methodName, args = null) {
        return "@M#" + methodName + (args !== null && args.length > 0 ? Fetch.RS + args.map(String).join(Fetch.US) : "");
    }

    // Date
    static DATE_YEAR = "@dy";
    // Month In JavaScript Is Start From Index 0, Month In WebForms Core Is Start From Index 1 
    static DATE_MONTH = "@dm";
    static DATE_DAY = "@dd";
    static DATE_DATE = "@dD";
    static DATE_HOURS = "@dh";
    static DATE_MINUTES = "@di";
    static DATE_SECONDS = "@ds";
    static DATE_MILLISECONDS = "@dl";

    // String
    static SPACE = "@sp";
    static AT_SIGN = "@sa";

    // Tag
    static getId(inputPlace) { return "@$i" + inputPlace; }
    static getName(inputPlace) { return "@$n" + inputPlace; }
    static getValue(inputPlace) { return "@$v" + inputPlace; }
    static getValueLength(inputPlace) { return "@$e" + inputPlace; }
    static getClass(inputPlace) { return "@$c" + inputPlace; }
    static getStyle(inputPlace) { return "@$s" + inputPlace; }
    static getTitle(inputPlace) { return "@$l" + inputPlace; }
    static getLabel(inputPlace) { return "@$A" + inputPlace; }
    static getText(inputPlace) { return "@$t" + inputPlace; }
    static getOuterText(inputPlace) { return "@$o" + inputPlace; }
    static getTextLength(inputPlace) { return "@$g" + inputPlace; }
    static getAttribute(inputPlace, attribute) { return "@$a" + inputPlace + Fetch.RS + attribute; }
    static getWidth(inputPlace) { return "@$w" + inputPlace; }
    static getHeight(inputPlace) { return "@$h" + inputPlace; }
    static getIsReadOnly(inputPlace) { return "@$r" + inputPlace; }
    static getSelectedIndex(inputPlace) { return "@$x" + inputPlace; }
    static getIndex(inputPlace) { return "@$I" + inputPlace; }
    static getTextAlign(inputPlace) { return "@$T" + inputPlace; }
    static getNodeLength(inputPlace) { return "@$L" + inputPlace; }
    static getIsVisible(inputPlace) { return "@$V" + inputPlace; }
	static getTagHash(inputPlace) { return "@$H" + inputPlace; }

    // Save and Cache
    static hasHash(hash) { return "@HH" + hash; }
    static cookie(key) { return "@co" + key; }
    static save(key = ".", replaceValue = null) { return "@cs" + key + (replaceValue !== null ? Fetch.RS + replaceValue : ""); }
    static saveThenRemove(key) { return "@cl" + key; }
    static saveLength(key = ".") { return "@cg" + key; }
    static cache(key = ".", replaceValue = null) { return "@cd" + key + (replaceValue !== null ? Fetch.RS + replaceValue : ""); }
    static cacheThenRemove(key) { return "@ct" + key; }
    static cacheLength(key = ".") { return "@cG" + key; }
    static saveLine(key = ".", line = 0) { return "@lL" + key + "[" + line; }
    static saveLineConsume(key = ".") { return "@lL" + key; }
    
    // INIKey: Only Direct Key is Supported
    static saveINI(key, iniKey) { return "@lI" + key + "[" + iniKey; }
    static cacheLine(key = ".", line = 0) { return "@dL" + key + "[" + line; }
    static cacheLineConsume(key = ".") { return "@dL" + key; }
    
    // INIKey: Only Direct Key is Supported
    static cacheINI(key, iniKey) { return "@dI" + key + "[" + iniKey; }

    // Format Storage
    static formatStore(key) { return "@fr" + key; }
    static formatStoreByXMLQuery(key, xpath) { return "@fx" + key + Fetch.RS + xpath; }
    static formatStoreByJSONQuery(key, query) { return "@fj" + key + Fetch.RS + query; }
    static formatStoreByINI(key, name) { return "@fi" + key + Fetch.RS + name; }
    static formatStoreByText(key, line) { return "@ft" + key + Fetch.RS + line; }
    static formatStoreByVariable(key) { return "@fv" + key; }

    // State
    static hasState(path) { return "@hs" + path; }

    // SSE
    static sseIsConnected(path) { return "@Sc" + path; }

    // WebSockets
    static webSocketsIsConnected(path = "") { return "@Wc" + path; }

    // Document
    static TAB_IS_ACTIVE = "@da";

    // Window
    static HREF = "@wf";
    static PATH_NAME = "@wP";
    static query(name = "*") { return "@wq" + name; }
    static HASH = "@wh";
    static HOST = "@wH";
    static HOST_NAME = "@wn";
    static PORT = "@wT";
    static ORIGIN = "@wo";
    static GET_SELECTION = "@ws";
    static SCROLL_X = "@wx";
    static SCROLL_Y = "@wy";
    static segment(index) { return "@wS" + index; }
    
    // It Only Works when the String Starts with the Tilde Character (~). The Path is Also Separated by the Slash Character (/). #~/Segment1/Segment2/Segment3
    static hashSegment(index) { return "@wt" + index; }

    // Navigator
    static CLIPBOARD_TEXT = "@nC";
    static GEO_LATITUDE = "@nW";
    static GEO_LONGITUDE = "@nO";
    static LANGUAGE = "@nL";
    static IS_ON_LINE = "@no";
    static USER_AGENT = "@na";

    // Screen
    static SCREEN_WIDTH = "@sw";
    static SCREEN_HEIGHT = "@sh";
    static SCREEN_ORIENTATION_TYPE = "@so";
    static SCREEN_ORIENTATION_ANGLE = "@sr";

    // Performance
    static TIME_ORIGIN = "@pt";
    static PERFORMANCE_NOW = "@pn";

    // Event
    static EVENT = "@EV";
    static EVENT_SERIALIZE = "@Es";
    static EVENT_KEY = "@ek";
    static EVENT_WHICH = "@ew";
    static EVENT_CLIENT_X = "@ex";
    static EVENT_CLIENT_Y = "@ey";
    static EVENT_PAGE_X = "@eX";
    static EVENT_PAGE_Y = "@eY";
    static EVENT_OFFSET_X = "@Ex";
    static EVENT_OFFSET_Y = "@Ey";
    static EVENT_DELTA_Y = "@ed";
}

export class WasmLanguage {
    static C = "c";
    static CPP = "c";
    static Rust = "rust";
    static CSharp = "csharp";
    static GO = "go";
    static JAVA = "java";
    static AssemblyScript = "as";
}

export class HtmlEvent {
    static OnAbort = "onabort";
    static OnAfterPrint = "onafterprint";
    static OnBeforePrint = "onbeforeprint";
    static OnBeforeUnload = "onbeforeunload";
    static OnBlur = "onblur";
    static OnCanPlay = "oncanplay";
    static OnCanPlayThrough = "oncanplaythrough";
    static OnChange = "onchange";
    static OnClick = "onclick";
    static OnCopy = "oncopy";
    static OnCut = "oncut";
    static OnDoubleClick = "ondblclick";
    static OnDrag = "ondrag";
    static OnDragEnd = "ondragend";
    static OnDragEnter = "ondragenter";
    static OnDragLeave = "ondragleave";
    static OnDragOver = "ondragover";
    static OnDragStart = "ondragstart";
    static OnDrop = "ondrop";
    static OnDurationChange = "ondurationchange";
    static OnEnded = "onended";
    static OnError = "onerror";
    static OnFocus = "onfocus";
    static OnFocusin = "onfocusin";
    static OnFocusOut = "onfocusout";
    static OnHashChange = "onhashchange";
    static OnInput = "oninput";
    static OnInvalid = "oninvalid";
    static OnKeyDown = "onkeydown";
    static OnKeyPress = "onkeypress";
    static OnKeyUp = "onkeyup";
    static OnLoad = "onload";
    static OnLoadedData = "onloadeddata";
    static OnLoadedMetaData = "onloadedmetadata";
    static OnLoadStart = "onloadstart";
    static OnMouseDown = "onmousedown";
    static OnMouseEnter = "onmouseenter";
    static OnMouseLeave = "onmouseleave";
    static OnMouseMove = "onmousemove";
    static OnMouseOver = "onmouseover";
    static OnMouseOut = "onmouseout";
    static OnMouseUp = "onmouseup";
    static OnOffline = "onoffline";
    static OnOnline = "ononline";
    static OnPageHide = "onpagehide";
    static OnPageShow = "onpageshow";
    static OnPaste = "onpaste";
    static OnPause = "onpause";
    static OnPlay = "onplay";
    static OnPlaying = "onplaying";
    static OnProgress = "onprogress";
    static OnRateChange = "onratechange";
    static OnResize = "onresize";
    static OnReset = "onreset";
    static OnScroll = "onscroll";
    static OnSearch = "onsearch";
    static OnSeeked = "onseeked";
    static OnSeeking = "onseeking";
    static OnSelect = "onselect";
    static OnStalled = "onstalled";
    static OnSubmit = "onsubmit";
    static OnSuspend = "onsuspend";
    static OnTimeUpdate = "ontimeupdate";
    static OnToggle = "ontoggle";
    static OnTouchCancel = "ontouchcancel";
    static OnTouchend = "ontouchend";
    static OnTouchMove = "ontouchmove";
    static OnTouchStart = "ontouchstart";
    static OnUnload = "onunload";
    static OnVolumeChange = "onvolumechange";
    static OnWaiting = "onwaiting";
    static OnWheel = "onwheel";
}

export class HtmlEventListener {
    static Abort = "abort";
    static AfterPrint = "afterprint";
    static BeforePrint = "beforeprint";
    static BeforeUnload = "beforeunload";
    static Blur = "blur";
    static CanPlay = "canplay";
    static CanPlayThrough = "canplaythrough";
    static Change = "change";
    static Click = "click";
    static Copy = "copy";
    static Cut = "cut";
    static DoubleClick = "dblclick";
    static Drag = "drag";
    static DragEnd = "dragend";
    static DragEnter = "dragenter";
    static DragLeave = "dragleave";
    static DragOver = "dragover";
    static DragStart = "dragstart";
    static Drop = "drop";
    static DurationChange = "durationchange";
    static Ended = "ended";
    static Error = "error";
    static Focus = "focus";
    static Focusin = "focusin";
    static FocusOut = "focusout";
    static HashChange = "hashchange";
    static Input = "input";
    static Invalid = "invalid";
    static KeyDown = "keydown";
    static KeyPress = "keypress";
    static KeyUp = "keyup";
    static Load = "load";
    static LoadedData = "loadeddata";
    static LoadedMetaData = "loadedmetadata";
    static LoadStart = "loadstart";
    static MouseDown = "mousedown";
    static MouseEnter = "mouseenter";
    static MouseLeave = "mouseleave";
    static MouseMove = "mousemove";
    static MouseOver = "mouseover";
    static MouseOut = "mouseout";
    static MouseUp = "mouseup";
    static Offline = "offline";
    static Online = "online";
    static PageHide = "pagehide";
    static PageShow = "pageshow";
    static Paste = "paste";
    static Pause = "pause";
    static Play = "play";
    static Playing = "playing";
    static Progress = "progress";
    static RateChange = "ratechange";
    static Resize = "resize";
    static Reset = "reset";
    static Scroll = "scroll";
    static Search = "search";
    static Seeked = "seeked";
    static Seeking = "seeking";
    static Select = "select";
    static Stalled = "stalled";
    static Submit = "submit";
    static Suspend = "suspend";
    static TimeUpdate = "timeupdate";
    static Toggle = "toggle";
    static TouchCancel = "touchcancel";
    static Touchend = "touchend";
    static TouchMove = "touchmove";
    static TouchStart = "touchstart";
    static Unload = "unload";
    static VolumeChange = "volumechange";
    static Waiting = "waiting";
    static Wheel = "wheel";

    static AnimationEnd = "animationend";
    static AnimationIteration = "animationiteration";
    static AnimationStart = "animationstart";
    static ContextMenu = "contextmenu";
    static FullScreenChange = "fullscreenchange";
    static FullScreenError = "fullscreenerror";
    static PopState = "popstate";
    static TransitionEnd = "transitionend";
    static Storage = "storage";

    // Custom
    static ScrollBottom = "scrollbottom"; // Need Call EnableScrollBottomEvent Method Before
    static ElementReached = "elementreached"; // Need Call EnableReachedElementEvent Method Before
}

export class ExtensionWebFormsMethods {
    static child(text, value) {
        if (text.length < 1) return value;
        return text + "|" + value;
    }

    static parent(text) {
        if (text.length < 1) return text;
        if (text.endsWith("|/") || text.endsWith("//")) return text + "/";
        return text + "|/";
    }

    static criteria(text, value) {
        if (text.length < 1) return value;
        return text + "?" + value.replace(/\|/g, "$[vb];").replace(/\?/g, "$[qu];").replace(/=/g, "$[eq];");
    }

    static appendFetchReplace(text, searchValue, value) {
        const fs = "\x1C";
        return "@;" + searchValue + fs + value + fs + (text.length > 0 ? text.substring(1) : "");
    }

    static lineBreak(text, encodeLine = false) {
        const encode = encodeLine ? "$[sln];" : "";
        return text.replace(/\r\n/g, encode).replace(/\n/g, encode).replace(/\r/g, encode);
    }

    // Converts Numbers to Strings
    static toJSString(text) { return '"' + text + '"'; }

    // Get JS Object Momentary 
    static toJSObject(text) { return "$" + text; }

    // Get JS Object Returned Value Once
    static toJSReturnObject(text) { return "$@" + text; }
}