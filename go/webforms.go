// webforms.go 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

package WebFormsCore

import (
	"fmt"
	"strconv"
	"strings"
)

func boolString(value bool) string {
	if value {
		return "true"
	}
	return "false"
}

func joinUS(values []any) string {
	result := make([]string, len(values))

	for i, value := range values {
		result[i] = fmt.Sprint(value)
	}

	return strings.Join(result, US)
}

type WebForms struct {
	webFormsData string
}

const (
	GS = string(rune(29))
	US = string(rune(31))
)

func New() *WebForms {
	return &WebForms{}
}

func (w *WebForms) add(name string, value ...string) {
	if w.webFormsData != "" {
		w.webFormsData += "\n"
	}

	w.webFormsData += name

	if len(value) > 0 {
		w.webFormsData += "=" + value[0]
	}
}

func (w *WebForms) addName(name string) {
	if len(w.webFormsData) > 0 {
		w.webFormsData += "\n"
	}

	w.webFormsData += name
}

func (w *WebForms) addToUp(name string, value ...string) {
	line := name

	if len(value) > 0 {
		line += "=" + value[0]
	}

	if len(w.webFormsData) > 0 {
		line += "\n"
	}

	w.webFormsData = line + w.webFormsData
}

func (w *WebForms) getLineByIndex(index int) string {
	if len(w.webFormsData) == 0 {
		return ""
	}

	data := w.webFormsData
	lines := strings.Split(data, "\n")

	if index < 0 {
		index = len(lines) + index
	}

	if index < 0 || index >= len(lines) {
		return ""
	}

	return lines[index]
}

func (w *WebForms) updateLineByIndex(index int, name string, value string) {
	if len(w.webFormsData) == 0 {
		return
	}

	data := w.webFormsData
	lines := strings.Split(data, "\n")

	if index < 0 {
		index = len(lines) + index
	}

	if index < 0 || index >= len(lines) {
		return
	}

	if value == "" {
		lines[index] = name
	} else {
		lines[index] = name + "=" + value
	}

	w.webFormsData = strings.Join(lines, "\n")
}

// For Extension
func (w *WebForms) AddLine(name string, value string) {
	w.add(name, value)
}

// Add
// Creates the Data if it does not exist; otherwise, Appends the New Value to the Existing Value.
func (w *WebForms) AddId(inputPlace string, id string) {
	w.add("ai"+inputPlace, id)
}

func (w *WebForms) AddName(inputPlace string, name string) {
	w.add("an"+inputPlace, name)
}

func (w *WebForms) AddValue(inputPlace string, value string) {
	w.add("av"+inputPlace, value)
}

func (w *WebForms) AddClass(inputPlace string, class string) {
	w.add("ac"+inputPlace, class)
}

func (w *WebForms) AddStyle(inputPlace string, args ...string) {
	if len(args) == 1 {
		w.add("as"+inputPlace, args[0])
		return
	}

	if len(args) == 2 {
		w.add("as"+inputPlace, args[0]+":"+args[1])
	}
}

func (w *WebForms) AddOptionTag(inputPlace string, text string, value string, selected ...bool) {
	isSelected := false

	if len(selected) > 0 {
		isSelected = selected[0]
	}

	data := value + GS + text

	if isSelected {
		data += GS + "1"
	}

	w.add("ao"+inputPlace, data)
}

func (w *WebForms) AddCheckBoxTag(inputPlace string, text string, value string, checked ...bool) {
	isChecked := false

	if len(checked) > 0 {
		isChecked = checked[0]
	}

	data := value + GS + text

	if isChecked {
		data += GS + "1"
	}

	w.add("ak"+inputPlace, data)
}

func (w *WebForms) AddTitle(inputPlace string, title string) {
	w.add("al"+inputPlace, title)
}

func (w *WebForms) AddLabel(inputPlace string, label string) {
	w.add("aA"+inputPlace, label)
}

func (w *WebForms) AddText(inputPlace string, text string) {
	w.add("at"+inputPlace, strings.ReplaceAll(text, "\n", "$[ln];"))
}

func (w *WebForms) AddTextToUp(inputPlace string, text string) {
	w.add("pt"+inputPlace, strings.ReplaceAll(text, "\n", "$[ln];"))
}

func (w *WebForms) AddAttribute(inputPlace string, attribute string, args ...any) {
	value := ""
	var splitter rune = '\x00'

	if len(args) > 0 {
		if v, ok := args[0].(string); ok {
			value = v
		}
	}

	if len(args) > 1 {
		switch v := args[1].(type) {
		case rune:
			splitter = v
		case byte:
			splitter = rune(v)
		case string:
			if len(v) > 0 {
				splitter = []rune(v)[0]
			}
		}
	}

	data := attribute + GS

	if splitter != '\x00' {
		data += string(splitter)
	}

	if value != "" {
		data += GS + value
	}

	w.add("aa"+inputPlace, data)
}

func (w *WebForms) AddTag(inputPlace string, tagName string, id ...string) {
	tagID := ""

	if len(id) > 0 {
		tagID = id[0]
	}

	data := tagName

	if tagID != "" {
		data += GS + tagID
	}

	w.add("nt"+inputPlace, data)
}

func (w *WebForms) AddTagToUp(inputPlace string, tagName string, id ...string) {
	tagID := ""

	if len(id) > 0 {
		tagID = id[0]
	}

	data := tagName

	if tagID != "" {
		data += GS + tagID
	}

	w.add("ut"+inputPlace, data)
}

func (w *WebForms) AddTagBefore(inputPlace string, tagName string, id ...string) {
	tagID := ""

	if len(id) > 0 {
		tagID = id[0]
	}

	data := tagName

	if tagID != "" {
		data += GS + tagID
	}

	w.add("bt"+inputPlace, data)
}

func (w *WebForms) AddTagAfter(inputPlace string, tagName string, id ...string) {
	tagID := ""

	if len(id) > 0 {
		tagID = id[0]
	}

	data := tagName

	if tagID != "" {
		data += GS + tagID
	}

	w.add("ft"+inputPlace, data)
}

func (w *WebForms) AddHidden(inputPlace string, name string, value string, id ...string) {
	tagID := ""

	if len(id) > 0 {
		tagID = id[0]
	}

	data := name + GS + value

	if tagID != "" {
		data += GS + tagID
	}

	w.add("ah"+inputPlace, data)
}

// Set
// Creates the Data if it does not exist; otherwise, Replaces the Existing Value with the New Value.
func (w *WebForms) SetId(inputPlace string, id string) {
	w.add("si"+inputPlace, id)
}

func (w *WebForms) SetName(inputPlace string, name string) {
	w.add("sn"+inputPlace, name)
}

func (w *WebForms) SetValue(inputPlace string, value string) {
	w.add("sv"+inputPlace, value)
}

func (w *WebForms) SetClass(inputPlace string, class string) {
	w.add("sc"+inputPlace, class)
}

func (w *WebForms) SetStyle(inputPlace string, args ...string) {
	if len(args) == 1 {
		w.add("ss"+inputPlace, args[0])
		return
	}

	if len(args) == 2 {
		w.add("ss"+inputPlace, args[0]+":"+args[1])
	}
}

func (w *WebForms) SetOptionTag(inputPlace string, text string, value string, selected ...bool) {
	isSelected := false

	if len(selected) > 0 {
		isSelected = selected[0]
	}

	data := value + GS + text

	if isSelected {
		data += GS + "1"
	}

	w.add("so"+inputPlace, data)
}

func (w *WebForms) SetChecked(inputPlace string, checked ...bool) {
	isChecked := false

	if len(checked) > 0 {
		isChecked = checked[0]
	}

	if isChecked {
		w.add("sk"+inputPlace, "1")
	} else {
		w.add("sk"+inputPlace, "0")
	}
}

func (w *WebForms) SetCheckBoxTag(inputPlace string, text string, value string, checked ...bool) {
	isChecked := false

	if len(checked) > 0 {
		isChecked = checked[0]
	}

	data := value + GS + text

	if isChecked {
		data += GS + "1"
	}

	w.add("sk"+inputPlace, data)
}

func (w *WebForms) SetTitle(inputPlace string, title string) {
	w.add("sl"+inputPlace, title)
}

func (w *WebForms) SetLabel(inputPlace string, label string) {
	w.add("sA"+inputPlace, label)
}

func (w *WebForms) SetText(inputPlace string, text string) {
	w.add("st"+inputPlace, strings.ReplaceAll(text, "\n", "$[ln];"))
}

func (w *WebForms) SetAttribute(inputPlace string, attribute string, value ...string) {
	attributeValue := ""

	if len(value) > 0 {
		attributeValue = value[0]
	}

	data := attribute + GS

	if attributeValue != "" {
		data += GS + attributeValue
	}

	w.add("sa"+inputPlace, data)
}

func (w *WebForms) SetWidth(inputPlace string, width any) {
	switch value := width.(type) {
	case string:
		w.add("sw"+inputPlace, value)
	case int:
		w.add("sw"+inputPlace, strconv.Itoa(value)+"px")
	}
}

func (w *WebForms) SetHeight(inputPlace string, height any) {
	switch value := height.(type) {
	case string:
		w.add("sh"+inputPlace, value)
	case int:
		w.add("sh"+inputPlace, strconv.Itoa(value)+"px")
	}
}

func (w *WebForms) SetBackgroundColor(inputPlace string, color string) {
	w.add("bc"+inputPlace, color)
}

func (w *WebForms) SetTextColor(inputPlace string, color string) {
	w.add("tc"+inputPlace, color)
}

func (w *WebForms) SetFontName(inputPlace string, name string) {
	w.add("fn"+inputPlace, name)
}

func (w *WebForms) SetFontSize(inputPlace string, size any) {
	switch value := size.(type) {
	case string:
		w.add("fs"+inputPlace, value)
	case int:
		w.add("fs"+inputPlace, strconv.Itoa(value)+"px")
	}
}

func (w *WebForms) SetFontBold(inputPlace string, bold bool) {
	if bold {
		w.add("fb"+inputPlace, "1")
	} else {
		w.add("fb"+inputPlace, "0")
	}
}

func (w *WebForms) SetVisible(inputPlace string, visible bool) {
	if visible {
		w.add("vi"+inputPlace, "1")
	} else {
		w.add("vi"+inputPlace, "0")
	}
}

func (w *WebForms) SetTextAlign(inputPlace string, align string) {
	w.add("ta"+inputPlace, align)
}

func (w *WebForms) SetReadOnly(inputPlace string, readOnly bool) {
	if readOnly {
		w.add("sr"+inputPlace, "1")
	} else {
		w.add("sr"+inputPlace, "0")
	}
}

func (w *WebForms) SetDisabled(inputPlace string, disabled bool) {
	if disabled {
		w.add("sd"+inputPlace, "1")
	} else {
		w.add("sd"+inputPlace, "0")
	}
}

func (w *WebForms) SetFocus(inputPlace string, focus bool) {
	if focus {
		w.add("sf"+inputPlace, "1")
	} else {
		w.add("sf"+inputPlace, "0")
	}
}

func (w *WebForms) SetMinLength(inputPlace string, length any) {
	switch value := length.(type) {
	case string:
		w.add("mn"+inputPlace, value)
	case int:
		w.add("mn"+inputPlace, strconv.Itoa(value))
	}
}

func (w *WebForms) SetMaxLength(inputPlace string, length any) {
	switch value := length.(type) {
	case string:
		w.add("mx"+inputPlace, value)
	case int:
		w.add("mx"+inputPlace, strconv.Itoa(value))
	}
}

func (w *WebForms) SetSelectedValue(inputPlace string, value string) {
	w.add("ts"+inputPlace, value)
}

func (w *WebForms) SetSelectedIndex(inputPlace string, index any) {
	switch value := index.(type) {
	case string:
		w.add("ti"+inputPlace, value)
	case int:
		w.add("ti"+inputPlace, strconv.Itoa(value))
	}
}

func (w *WebForms) SetCheckedValue(inputPlace string, value string, checked bool) {
	if checked {
		w.add("ks"+inputPlace, value+GS+"1")
	} else {
		w.add("ks"+inputPlace, value+GS+"0")
	}
}

func (w *WebForms) SetCheckedIndex(inputPlace string, index any, checked bool) {
	indexValue := ""

	switch value := index.(type) {
	case string:
		indexValue = value
	case int:
		indexValue = strconv.Itoa(value)
	}

	if checked {
		w.add("ki"+inputPlace, indexValue+GS+"1")
	} else {
		w.add("ki"+inputPlace, indexValue+GS+"0")
	}
}

func (w *WebForms) SetCustomValidity(inputPlace string, text string) {
	w.add("cv"+inputPlace, strings.ReplaceAll(text, "\n", "$[ln];"))
}

// Insert
// Creates the Data only if it does not exist; otherwise, does nothing.
func (w *WebForms) InsertId(inputPlace string, id string) {
	w.add("ii"+inputPlace, id)
}

func (w *WebForms) InsertName(inputPlace string, name string) {
	w.add("in"+inputPlace, name)
}

func (w *WebForms) InsertValue(inputPlace string, value string) {
	w.add("iv"+inputPlace, value)
}

func (w *WebForms) InsertClass(inputPlace string, class string) {
	w.add("ic"+inputPlace, class)
}

func (w *WebForms) InsertStyle(inputPlace string, args ...string) {
	if len(args) == 1 {
		w.add("is"+inputPlace, args[0])
		return
	}

	if len(args) == 2 {
		w.add("is"+inputPlace, args[0]+":"+args[1])
	}
}

func (w *WebForms) InsertOptionTag(inputPlace string, text string, value string, selected ...bool) {
	isSelected := false

	if len(selected) > 0 {
		isSelected = selected[0]
	}

	data := value + GS + text

	if isSelected {
		data += GS + "1"
	}

	w.add("io"+inputPlace, data)
}

func (w *WebForms) InsertCheckBoxTag(inputPlace string, text string, value string, checked ...bool) {
	isChecked := false

	if len(checked) > 0 {
		isChecked = checked[0]
	}

	data := value + GS + text

	if isChecked {
		data += GS + "1"
	}

	w.add("ik"+inputPlace, data)
}

func (w *WebForms) InsertTitle(inputPlace string, title string) {
	w.add("il"+inputPlace, title)
}

func (w *WebForms) InsertLabel(inputPlace string, label string) {
	w.add("iA"+inputPlace, label)
}

func (w *WebForms) InsertText(inputPlace string, text string) {
	w.add("it"+inputPlace, strings.ReplaceAll(text, "\n", "$[ln];"))
}

func (w *WebForms) InsertAttribute(inputPlace string, attribute string, args ...any) {
	value := ""
	var splitter rune = '\x00'

	if len(args) > 0 {
		if v, ok := args[0].(string); ok {
			value = v
		}
	}

	if len(args) > 1 {
		switch v := args[1].(type) {
		case rune:
			splitter = v
		case byte:
			splitter = rune(v)
		case string:
			if len(v) > 0 {
				splitter = []rune(v)[0]
			}
		}
	}

	data := attribute + GS

	if splitter != '\x00' {
		data += string(splitter)
	}

	if value != "" {
		data += GS + value
	}

	w.add("ia"+inputPlace, data)
}

// Delete
func (w *WebForms) DeleteId(inputPlace string) {
	w.addName("di" + inputPlace)
}

func (w *WebForms) DeleteName(inputPlace string) {
	w.addName("dn" + inputPlace)
}

func (w *WebForms) DeleteValue(inputPlace string) {
	w.addName("dv" + inputPlace)
}

func (w *WebForms) DeleteClass(inputPlace string, className string) {
	w.add("dc"+inputPlace, className)
}

func (w *WebForms) DeleteStyle(inputPlace string, styleName string) {
	w.add("ds"+inputPlace, styleName)
}

func (w *WebForms) DeleteOptionTag(inputPlace string, value string) {
	w.add("do"+inputPlace, value)
}

func (w *WebForms) DeleteAllOptionTag(inputPlace string) {
	w.add("do"+inputPlace, "*")
}

func (w *WebForms) DeleteCheckBoxTag(inputPlace string, value string) {
	w.add("dk"+inputPlace, value)
}

func (w *WebForms) DeleteAllCheckBoxTag(inputPlace string) {
	w.add("dk"+inputPlace, "*")
}

func (w *WebForms) DeleteTitle(inputPlace string) {
	w.addName("dl" + inputPlace)
}

func (w *WebForms) DeleteLabel(inputPlace string) {
	w.addName("dA" + inputPlace)
}

func (w *WebForms) DeleteText(inputPlace string) {
	w.addName("dt" + inputPlace)
}

func (w *WebForms) DeleteAttribute(inputPlace string, attribute string) {
	w.add("da"+inputPlace, attribute)
}

func (w *WebForms) Delete(inputPlace string) {
	w.addName("de" + inputPlace)
}

func (w *WebForms) DeleteParent(inputPlace string) {
	w.addName("dp" + inputPlace)
}

// Tag Transformation
func (w *WebForms) SwapTag(inputPlace string, outputPlace string) {
	w.add("sp"+inputPlace, outputPlace)
}

func (w *WebForms) SetReflect(inputPlace string, tag string) {
	w.add("sR"+inputPlace, tag)
}

func (w *WebForms) SetReflectByOutputPlace(inputPlace string, outputPlace string) {
	w.add("iR"+inputPlace, outputPlace)
}

func (w *WebForms) SetMorph(inputPlace string, tag string) {
	w.add("sM"+inputPlace, tag)
}

func (w *WebForms) SetMorphByOutputPlace(inputPlace string, outputPlace string) {
	w.add("iM"+inputPlace, outputPlace)
}

func (w *WebForms) Snapshot(inputPlace string, args ...any) {
	key := ""
	permanent := false

	if len(args) > 0 {
		if k, ok := args[0].(string); ok {
			key = k
		}
	}
	if len(args) > 1 {
		if p, ok := args[1].(bool); ok {
			permanent = p
		}
	}

	if key == "" {
		key = inputPlace
	}
	if permanent {
		key += GS + "1"
	}

	w.add("rS"+inputPlace, key)
}

func (w *WebForms) Rollback(inputPlace string, args ...any) {
	key := ""
	permanent := false

	if len(args) > 0 {
		if k, ok := args[0].(string); ok {
			key = k
		}
	}
	if len(args) > 1 {
		if p, ok := args[1].(bool); ok {
			permanent = p
		}
	}

	if key == "" {
		key = inputPlace
	}
	if permanent {
		key += GS + "1"
	}

	w.add("rB"+inputPlace, key)
}

// Browser
func (w *WebForms) ChangeUrl(url string) {
	w.add("cu", url)
}

func (w *WebForms) SetHeadTitle(title string) {
	w.add("ht", title)
}

func (w *WebForms) ClipboardWriteText(text string) {
	w.add("nw", text)
}

func (w *WebForms) ScrollTo(x any, y any) {
	var xValue string
	var yValue string

	switch value := x.(type) {
	case string:
		xValue = value
	case int:
		xValue = strconv.Itoa(value)
	}

	switch value := y.(type) {
	case string:
		yValue = value
	case int:
		yValue = strconv.Itoa(value)
	}

	w.add("ws", xValue+GS+yValue)
}

func (w *WebForms) HistoryGo(steps any) {
	switch value := steps.(type) {
	case string:
		w.add("wg", value)
	case int:
		w.add("wg", strconv.Itoa(value))
	}
}

func (w *WebForms) ReloadPage() {
	w.addName("lr")
}

func (w *WebForms) Redirect(path string) {
	w.add("lh", path)
}

// Increase
func (w *WebForms) IncreaseMinLength(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("+n"+inputPlace, v)
	case int:
		w.add("+n"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) IncreaseMaxLength(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("+x"+inputPlace, v)
	case int:
		w.add("+x"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) IncreaseFontSize(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("+f"+inputPlace, v)
	case int:
		w.add("+f"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) IncreaseWidth(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("+w"+inputPlace, v)
	case int:
		w.add("+w"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) IncreaseHeight(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("+h"+inputPlace, v)
	case int:
		w.add("+h"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) IncreaseValue(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("+v"+inputPlace, v)
	case int:
		w.add("+v"+inputPlace, strconv.Itoa(v))
	}
}

// Decrease
func (w *WebForms) DecreaseMinLength(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("-n"+inputPlace, v)
	case int:
		w.add("-n"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) DecreaseMaxLength(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("-x"+inputPlace, v)
	case int:
		w.add("-x"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) DecreaseFontSize(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("-f"+inputPlace, v)
	case int:
		w.add("-f"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) DecreaseWidth(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("-w"+inputPlace, v)
	case int:
		w.add("-w"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) DecreaseHeight(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("-h"+inputPlace, v)
	case int:
		w.add("-h"+inputPlace, strconv.Itoa(v))
	}
}

func (w *WebForms) DecreaseValue(inputPlace string, value any) {
	switch v := value.(type) {
	case string:
		w.add("-v"+inputPlace, v)
	case int:
		w.add("-v"+inputPlace, strconv.Itoa(v))
	}
}

// Event
// ConstructorName: mouseevent, keyboardevent, uievent, focusevent, inputevent, event
// All Method in "Event" Section Only Support Dynamic Args Once. To Support Invoking Dynamic Arguments on a Momentary Basis, Use "EventListener" Section Methods.
func (w *WebForms) TriggerEvent(inputPlace string, htmlEventListener string, constructorName ...string) {
	name := ""

	if len(constructorName) > 0 {
		name = constructorName[0]
	}

	value := htmlEventListener

	if name != "" {
		value += GS + name
	}

	w.add("TE"+inputPlace, value)
}

func (w *WebForms) SetPostEvent(inputPlace string, htmlEvent string, outputPlace ...string) {
	value := htmlEvent

	if len(outputPlace) > 0 {
		value += GS + outputPlace[0]
	}

	w.add("Ep"+inputPlace, value)
}

func (w *WebForms) SetPostEventAddView(inputPlace string, htmlEvent string) {
	w.add("Ep"+inputPlace, htmlEvent+GS+"+")
}

func (w *WebForms) SetPostEventListener(inputPlace string, htmlEventListener string, outputPlace ...string) {
	value := htmlEventListener

	if len(outputPlace) > 0 {
		value += GS + outputPlace[0]
	}

	w.add("EP"+inputPlace, value)
}

func (w *WebForms) SetPostEventListenerAddView(inputPlace string, htmlEventListener string) {
	w.add("EP"+inputPlace, htmlEventListener+GS+"+")
}

func (w *WebForms) SetGetEvent(inputPlace string, htmlEvent string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEvent + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("Eg"+inputPlace, value)
}

func (w *WebForms) SetGetEventListener(inputPlace string, htmlEventListener string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEventListener + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("EG"+inputPlace, value)
}

func (w *WebForms) SetPutEvent(inputPlace string, htmlEvent string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEvent + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("Et"+inputPlace, value)
}

func (w *WebForms) SetPutEventListener(inputPlace string, htmlEventListener string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEventListener + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("ET"+inputPlace, value)
}

func (w *WebForms) SetPatchEvent(inputPlace string, htmlEvent string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEvent + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("Ea"+inputPlace, value)
}

func (w *WebForms) SetPatchEventListener(inputPlace string, htmlEventListener string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEventListener + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("EA"+inputPlace, value)
}

func (w *WebForms) SetDeleteEvent(inputPlace string, htmlEvent string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEvent + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("El"+inputPlace, value)
}

func (w *WebForms) SetDeleteEventListener(inputPlace string, htmlEventListener string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEventListener + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("EL"+inputPlace, value)
}

func (w *WebForms) SetOptionsEvent(inputPlace string, htmlEvent string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEvent + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("Eo"+inputPlace, value)
}

func (w *WebForms) SetOptionsEventListener(inputPlace string, htmlEventListener string, args ...string) {
	path := "#"
	outputPlace := ""

	if len(args) == 1 {
		if args[0] != "" {
			path = args[0]
		}
	}

	if len(args) >= 2 {
		outputPlace = args[0]

		if args[1] != "" {
			path = args[1]
		}
	}

	value := htmlEventListener + GS + path

	if len(args) >= 2 {
		value += GS + outputPlace
	}

	w.add("EO"+inputPlace, value)
}

func (w *WebForms) SetHeadEvent(inputPlace string, htmlEvent string, path ...string) {
	finalPath := "#"

	if len(path) > 0 && path[0] != "" {
		finalPath = path[0]
	}

	w.add("Eh"+inputPlace, htmlEvent+GS+finalPath)
}

func (w *WebForms) SetHeadEventListener(inputPlace string, htmlEventListener string, path ...string) {
	finalPath := "#"

	if len(path) > 0 && path[0] != "" {
		finalPath = path[0]
	}

	w.add("EH"+inputPlace, htmlEventListener+GS+finalPath)
}

// IsMultiPart: If this value is true, the data will be sent based on the Form and with the "content" key.
func (w *WebForms) SetSendEvent(inputPlace string, htmlEvent string, data string, args ...any) {
	path := "#"
	method := "POST"
	isMultiPart := false
	contentType := "text/plain"
	outputPlace := ""

	if len(args) > 0 {
		if value, ok := args[0].(string); ok && value != "" {
			path = value
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			method = value
		}
	}

	if len(args) > 2 {
		if value, ok := args[2].(bool); ok {
			isMultiPart = value
		}
	}

	if len(args) > 3 {
		if value, ok := args[3].(string); ok {
			contentType = value
		}
	}

	if len(args) > 4 {
		if value, ok := args[4].(string); ok {
			outputPlace = value
		}
	}

	data = strings.ReplaceAll(data, "\n", "$[ln];")
	data = strings.ReplaceAll(data, "\"", "$[dq];")
	data = strings.ReplaceAll(data, "'", "$[sq];")

	multipartValue := "0"

	if isMultiPart {
		multipartValue = "1"
	}

	w.add(
		"En"+inputPlace,
		htmlEvent+
			GS+
			data+
			GS+
			path+
			GS+
			method+
			GS+
			multipartValue+
			GS+
			contentType+
			GS+
			outputPlace,
	)
}

func (w *WebForms) SetSendEventListener(inputPlace string, htmlEventListener string, data string, args ...any) {
	path := "#"
	method := "POST"
	isMultiPart := false
	contentType := "text/plain"
	outputPlace := ""

	if len(args) > 0 {
		if value, ok := args[0].(string); ok && value != "" {
			path = value
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			method = value
		}
	}

	if len(args) > 2 {
		if value, ok := args[2].(bool); ok {
			isMultiPart = value
		}
	}

	if len(args) > 3 {
		if value, ok := args[3].(string); ok {
			contentType = value
		}
	}

	if len(args) > 4 {
		if value, ok := args[4].(string); ok {
			outputPlace = value
		}
	}

	data = strings.ReplaceAll(data, "\n", "$[ln];")

	multipartValue := "0"

	if isMultiPart {
		multipartValue = "1"
	}

	w.add(
		"EN"+inputPlace,
		htmlEventListener+
			GS+
			data+
			GS+
			path+
			GS+
			method+
			GS+
			multipartValue+
			GS+
			contentType+
			GS+
			outputPlace,
	)
}

func (w *WebForms) SetCommentEvent(inputPlace string, htmlEvent string, args ...any) {
	index := ""
	outputPlace := ""

	if len(args) > 0 {
		switch value := args[0].(type) {
		case string:
			index = value
		case int:
			index = strconv.Itoa(value)
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			outputPlace = value
		}
	}

	w.add("Eb"+inputPlace, htmlEvent+GS+index+GS+outputPlace)
}

func (w *WebForms) SetCommentEventListener(inputPlace string, htmlEventListener string, args ...any) {
	index := ""
	outputPlace := ""

	if len(args) > 0 {
		switch value := args[0].(type) {
		case string:
			index = value
		case int:
			index = strconv.Itoa(value)
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			outputPlace = value
		}
	}

	w.add("EB"+inputPlace, htmlEventListener+GS+index+GS+outputPlace)
}

func joinArgs(args []any) string {
	if len(args) == 0 {
		return ""
	}

	values := make([]string, len(args))

	for i, arg := range args {
		values[i] = fmt.Sprint(arg)
	}

	return strings.Join(values, US)
}

func (w *WebForms) SetWasmEvent(inputPlace string, htmlEvent string, wasmLanguage string, wasmURL string, methodName string, args ...any) {
	var methodArgs []any
	outputPlace := ""

	if len(args) > 0 {
		if value, ok := args[0].([]any); ok {
			methodArgs = value
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			outputPlace = value
		}
	}

	argsJoin := ""

	if methodArgs != nil {
		if len(methodArgs) > 0 {
			argsJoin = "[" + joinArgs(methodArgs)
		}
	}

	w.add(
		"Ey"+inputPlace,
		htmlEvent+
			GS+
			wasmLanguage+
			GS+
			wasmURL+
			GS+
			methodName+
			GS+
			argsJoin+
			GS+
			outputPlace,
	)
}

func (w *WebForms) SetWasmEventListener(inputPlace string, htmlEventListener string, wasmLanguage string, wasmURL string, methodName string, args ...any) {
	var methodArgs []any
	outputPlace := ""

	if len(args) > 0 {
		if value, ok := args[0].([]any); ok {
			methodArgs = value
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			outputPlace = value
		}
	}

	argsJoin := ""

	if methodArgs != nil {
		if len(methodArgs) > 0 {
			argsJoin = "[" + joinArgs(methodArgs)
		}
	}

	w.add(
		"EY"+inputPlace,
		htmlEventListener+
			GS+
			wasmLanguage+
			GS+
			wasmURL+
			GS+
			methodName+
			GS+
			argsJoin+
			GS+
			outputPlace,
	)
}

func (w *WebForms) SetWebSocketEvent(inputPlace string, htmlEvent string, path string) {
	w.add("Ew"+inputPlace, htmlEvent+GS+path)
}

func (w *WebForms) SetWebSocketEventListener(inputPlace string, htmlEventListener string, path string) {
	w.add("EW"+inputPlace, htmlEventListener+GS+path)
}

func (w *WebForms) SetSSEEvent(inputPlace string, htmlEvent string, path string, args ...any) {
	outputPlace := ""
	shouldReconnect := true
	reconnectTryTimeout := 3000

	if len(args) > 0 {
		switch value := args[0].(type) {
		case string:
			outputPlace = value
		case bool:
			shouldReconnect = value
		}
	}

	if len(args) > 1 {
		switch value := args[1].(type) {
		case bool:
			shouldReconnect = value
		case int:
			reconnectTryTimeout = value
		}
	}

	if len(args) > 2 {
		if value, ok := args[2].(int); ok {
			reconnectTryTimeout = value
		}
	}

	reconnectValue := "0"

	if shouldReconnect {
		reconnectValue = "1"
	}

	data := htmlEvent +
		GS +
		path +
		GS +
		reconnectValue +
		GS +
		strconv.Itoa(reconnectTryTimeout)

	if outputPlace != "" {
		data += GS + outputPlace
	}

	w.add("Ee"+inputPlace, data)
}

func (w *WebForms) SetSSEEventListener(inputPlace string, htmlEventListener string, path string, args ...any) {
	outputPlace := ""
	shouldReconnect := true
	reconnectTryTimeout := 3000

	if len(args) > 0 {
		switch value := args[0].(type) {
		case string:
			outputPlace = value
		case bool:
			shouldReconnect = value
		}
	}

	if len(args) > 1 {
		switch value := args[1].(type) {
		case bool:
			shouldReconnect = value
		case int:
			reconnectTryTimeout = value
		}
	}

	if len(args) > 2 {
		if value, ok := args[2].(int); ok {
			reconnectTryTimeout = value
		}
	}

	reconnectValue := "0"

	if shouldReconnect {
		reconnectValue = "1"
	}

	data := htmlEventListener +
		GS +
		path +
		GS +
		reconnectValue +
		GS +
		strconv.Itoa(reconnectTryTimeout)

	if outputPlace != "" {
		data += GS + outputPlace
	}

	w.add("EE"+inputPlace, data)
}

func (w *WebForms) SetFrontEvent(inputPlace string, htmlEvent string, modulePath string, args ...any) {
	var methodArgs []any
	outputPlace := ""

	if len(args) > 0 {
		if value, ok := args[0].([]any); ok {
			methodArgs = value
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			outputPlace = value
		}
	}

	argsJoin := ""

	if methodArgs != nil && len(methodArgs) > 0 {
		argsJoin = GS + "[" + joinArgs(methodArgs)
	}

	w.add(
		"Ej"+inputPlace,
		htmlEvent+
			GS+
			modulePath+
			GS+
			outputPlace+
			argsJoin,
	)
}

func (w *WebForms) SetFrontEventListener(inputPlace string, htmlEventListener string, modulePath string, args ...any) {
	var methodArgs []any
	outputPlace := ""

	if len(args) > 0 {
		if value, ok := args[0].([]any); ok {
			methodArgs = value
		}
	}

	if len(args) > 1 {
		if value, ok := args[1].(string); ok {
			outputPlace = value
		}
	}

	argsJoin := ""

	if methodArgs != nil && len(methodArgs) > 0 {
		argsJoin = GS + "[" + joinArgs(methodArgs)
	}

	w.add(
		"EJ"+inputPlace,
		htmlEventListener+
			GS+
			modulePath+
			GS+
			outputPlace+
			argsJoin,
	)
}

func (w *WebForms) SetMasterPagesEvent(inputPlace string, htmlEvent string, outputPlace ...string) {
	value := ""

	if len(outputPlace) > 0 {
		value = outputPlace[0]
	}

	w.add("Eu"+inputPlace, htmlEvent+GS+value)
}

func (w *WebForms) SetMasterPagesEventListener(inputPlace string, htmlEventListener string, outputPlace ...string) {
	value := ""

	if len(outputPlace) > 0 {
		value = outputPlace[0]
	}

	w.add("EU"+inputPlace, htmlEventListener+GS+value)
}

func (w *WebForms) SetPreventDefaultEvent(inputPlace string, htmlEvent string) {
	w.add("Ed"+inputPlace, htmlEvent)
}

func (w *WebForms) SetPreventDefaultEventListener(inputPlace string, htmlEventListener string) {
	w.add("ED"+inputPlace, htmlEventListener)
}

func (w *WebForms) SetStopPropagationEvent(inputPlace string, htmlEvent string) {
	w.add("Es"+inputPlace, htmlEvent)
}

func (w *WebForms) SetStopPropagationEventListener(inputPlace string, htmlEventListener string) {
	w.add("ES"+inputPlace, htmlEventListener)
}

func (w *WebForms) SetMethodEvent(inputPlace string, htmlEvent string, methodName string, args ...any) {
	argsJoin := ""

	if len(args) > 0 {
		if methodArgs, ok := args[0].([]any); ok && len(methodArgs) > 0 {
			argsJoin = GS + "[" + joinArgs(methodArgs)
		}
	}

	w.add("Em"+inputPlace, htmlEvent+GS+methodName+argsJoin)
}

func (w *WebForms) SetMethodEventListener(inputPlace string, htmlEventListener string, methodName string, args ...any) {
	argsJoin := ""

	if len(args) > 0 {
		if methodArgs, ok := args[0].([]any); ok && len(methodArgs) > 0 {
			argsJoin = GS + "[" + joinArgs(methodArgs)
		}
	}

	w.add("EM"+inputPlace, htmlEventListener+GS+methodName+argsJoin)
}

func (w *WebForms) SetModuleMethodEvent(inputPlace string, htmlEvent string, methodName string, args ...any) {
	argsJoin := ""

	if len(args) > 0 {
		if methodArgs, ok := args[0].([]any); ok && len(methodArgs) > 0 {
			argsJoin = GS + "[" + joinArgs(methodArgs)
		}
	}

	w.add("Ex"+inputPlace, htmlEvent+GS+methodName+argsJoin)
}

func (w *WebForms) SetModuleMethodEventListener(inputPlace string, htmlEventListener string, methodName string, args ...any) {
	argsJoin := ""

	if len(args) > 0 {
		if methodArgs, ok := args[0].([]any); ok && len(methodArgs) > 0 {
			argsJoin = GS + "[" + joinArgs(methodArgs)
		}
	}

	w.add("EX"+inputPlace, htmlEventListener+GS+methodName+argsJoin)
}

func (w *WebForms) AssignConfirmEvent(inputPlace string, htmlEvent string, args ...string) {
	text := "Are you sure you want to proceed?"
	messageType := "none"
	title := "Confirm"
	okText := "OK"
	cancelText := "Cancel"

	if len(args) > 0 {
		text = args[0]
	}

	if len(args) > 1 {
		messageType = args[1]
	}

	if len(args) > 2 {
		title = args[2]
	}

	if len(args) > 3 {
		okText = args[3]
	}

	if len(args) > 4 {
		cancelText = args[4]
	}

	if text == "Are you sure you want to proceed?" {
		text = ""
	}

	if messageType == "none" {
		messageType = ""
	}

	if title == "Confirm" {
		title = ""
	}

	if okText == "OK" {
		okText = ""
	}

	if cancelText == "Cancel" {
		cancelText = ""
	}

	w.add(
		"Ef"+inputPlace,
		htmlEvent+
			GS+
			text+
			GS+
			messageType+
			GS+
			title+
			GS+
			okText+
			GS+
			cancelText,
	)
}

func (w *WebForms) RemovePostEvent(inputPlace string, htmlEvent string) {
	w.add("Rp"+inputPlace, htmlEvent)
}

func (w *WebForms) RemovePostEventListener(inputPlace string, htmlEventListener string) {
	w.add("RP"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveGetEvent(inputPlace string, htmlEvent string) {
	w.add("Rg"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveGetEventListener(inputPlace string, htmlEventListener string) {
	w.add("RG"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemovePutEvent(inputPlace string, htmlEvent string) {
	w.add("Rt"+inputPlace, htmlEvent)
}

func (w *WebForms) RemovePutEventListener(inputPlace string, htmlEventListener string) {
	w.add("RT"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemovePatchEvent(inputPlace string, htmlEvent string) {
	w.add("Ra"+inputPlace, htmlEvent)
}

func (w *WebForms) RemovePatchEventListener(inputPlace string, htmlEventListener string) {
	w.add("RA"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveDeleteEvent(inputPlace string, htmlEvent string) {
	w.add("Rl"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveDeleteEventListener(inputPlace string, htmlEventListener string) {
	w.add("RL"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveOptionsEvent(inputPlace string, htmlEvent string) {
	w.add("Ro"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveOptionsEventListener(inputPlace string, htmlEventListener string) {
	w.add("RO"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveHeadEvent(inputPlace string, htmlEvent string) {
	w.add("Rh"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveHeadEventListener(inputPlace string, htmlEventListener string) {
	w.add("RH"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveSendEvent(inputPlace string, htmlEvent string) {
	w.add("Rn"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveSendEventListener(inputPlace string, htmlEventListener string) {
	w.add("RN"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveCommentEvent(inputPlace string, htmlEvent string) {
	w.add("Rb"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveCommentEventListener(inputPlace string, htmlEventListener string) {
	w.add("RB"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveWasmEvent(inputPlace string, htmlEvent string) {
	w.add("Ry"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveWasmEventListener(inputPlace string, htmlEventListener string) {
	w.add("RY"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveWebSocketEvent(inputPlace string, htmlEvent string) {
	w.add("Rw"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveWebSocketEventListener(inputPlace string, htmlEventListener string) {
	w.add("RW"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveSSEEvent(inputPlace string, htmlEvent string) {
	w.add("Re"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveSSEEventListener(inputPlace string, htmlEventListener string) {
	w.add("RE"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveFrontEvent(inputPlace string, htmlEvent string) {
	w.add("Rj"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveFrontEventListener(inputPlace string, htmlEventListener string) {
	w.add("RJ"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemovePreventDefaultEvent(inputPlace string, htmlEvent string) {
	w.add("Rd"+inputPlace, htmlEvent)
}

func (w *WebForms) RemovePreventDefaultEventListener(inputPlace string, htmlEventListener string) {
	w.add("RD"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveMasterPagesEvent(inputPlace string, htmlEvent string) {
	w.add("Ru"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveMasterPagesEventListener(inputPlace string, htmlEventListener string) {
	w.add("RU"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveStopPropagationEvent(inputPlace string, htmlEvent string) {
	w.add("Rs"+inputPlace, htmlEvent)
}

func (w *WebForms) RemoveStopPropagationEventListener(inputPlace string, htmlEventListener string) {
	w.add("RS"+inputPlace, htmlEventListener)
}

func (w *WebForms) RemoveMethodEvent(inputPlace string, htmlEvent string, methodName string) {
	w.add("Rm"+inputPlace, htmlEvent+GS+methodName)
}

func (w *WebForms) RemoveMethodEventListener(inputPlace string, htmlEventListener string, methodName string) {
	w.add("RM"+inputPlace, htmlEventListener+GS+methodName)
}

func (w *WebForms) RemoveModuleMethodEvent(inputPlace string, htmlEvent string, methodName string) {
	w.add("Rx"+inputPlace, htmlEvent+GS+methodName)
}

func (w *WebForms) RemoveModuleMethodEventListener(inputPlace string, htmlEventListener string, methodName string) {
	w.add("RX"+inputPlace, htmlEventListener+GS+methodName)
}

func (w *WebForms) RemoveConfirmEvent(inputPlace string, htmlEvent string) {
	w.add("Rf"+inputPlace, htmlEvent)
}

// Custom Event
// This Method Is Compatible With EventListener And May Not Be Compatible With Events Written As Attributes In Some Browsers.
// Watch: attribute, style, text, children, value
// Compare: greater, less, equal, notequal, includes, startswith, endswith, matches, changed, inrange, lengthgreater, lengthless, lengthequal
// Range: Only Use For Compare With inrange Value. Split By Comma ","
// Key: Only Use For Watch With attribute And style Value
func (w *WebForms) CreateCustomDOMEvent(inputPlace string, eventName string, watch string, key string, compare string, value string, rangeValue string, immediate bool, delay any) {
	delayValue := ""

	switch value := delay.(type) {
	case string:
		delayValue = value
	case int:
		delayValue = strconv.Itoa(value)
	}

	immediateValue := "0"
	if immediate {
		immediateValue = "1"
	}

	w.add(
		"eC"+inputPlace,
		eventName+
			GS+
			watch+
			GS+
			key+
			GS+
			compare+
			GS+
			value+
			GS+
			rangeValue+
			GS+
			immediateValue+
			GS+
			delayValue,
	)
}

func (w *WebForms) EnableScrollBottomEvent(enable ...bool) {
	value := true

	if len(enable) > 0 {
		value = enable[0]
	}

	if value {
		w.add("eb", "1")
	} else {
		w.add("eb", "0")
	}
}

func (w *WebForms) EnableReachedElementEvent(inputPlace string, once bool, enable ...bool) {
	value := true

	if len(enable) > 0 {
		value = enable[0]
	}

	onceValue := "0"
	enableValue := "0"

	if once {
		onceValue = "1"
	}

	if value {
		enableValue = "1"
	}

	w.add("er"+inputPlace, onceValue+GS+enableValue)
}

// Module
func (w *WebForms) LoadModule(modulePath string, methods ...[]string) {
	var methodList []string

	if len(methods) > 0 {
		methodList = methods[0]
	}

	value := modulePath

	if len(methodList) > 0 {
		value += GS + "[" + strings.Join(methodList, US)
	}

	w.add("Ml", value)
}

func (w *WebForms) UnloadModule(modulePath string) {
	w.add("Mu", modulePath)
}

func (w *WebForms) DeleteModuleMethod(methodName string) {
	w.add("Md", methodName)
}

// Unit Testing
// InputPlace Is Actual, Expected Is Tag/OutputPlace
func (w *WebForms) AssertEqual(inputPlace string, tag string) {
	w.add("At"+inputPlace, strings.ReplaceAll(tag, "\n", "$[ln];"))
}

func (w *WebForms) AssertEqualByOutputPlace(inputPlace string, outputPlace string) {
	w.add("Ao"+inputPlace, outputPlace)
}

// Debug
func (w *WebForms) CreateDebugger(pause ...bool) {
	value := false

	if len(pause) > 0 {
		value = pause[0]
	}

	if value {
		w.add("Dc", "1")
	} else {
		w.add("Dc", "0")
	}
}

func (w *WebForms) Try() {
	w.add("tr")
}

func (w *WebForms) Catch() {
	w.add("ca")
}

func (w *WebForms) Comment(text string) {
	w.add("//", text)
}

// Service Worker
// To Use Service Worker, You Need To Add The Elanat Dedicated Module (service-worker.js) On The Client Side
func (w *WebForms) ServiceWorkerRegister(path ...string) {
	pathValue := ""
	scopePath := ""

	if len(path) > 0 {
		pathValue = path[0]
	}

	if len(path) > 1 {
		scopePath = path[1]
	}

	w.add("wR", pathValue+GS+scopePath)
}

func (w *WebForms) ServiceWorkerPreCacheStatic(pathList []string) {
	w.add("wp", strings.Join(pathList, GS))
}

func (w *WebForms) ServiceWorkerDynamicCache(path string, seconds ...any) {
	secondsValue := ""

	if len(seconds) > 0 {
		switch value := seconds[0].(type) {
		case string:
			secondsValue = value
		case int:
			if value > 0 {
				secondsValue = strconv.Itoa(value)
			}
		}
	}

	w.add("wc", path+(func() string {
		if secondsValue != "" {
			return GS + secondsValue
		}

		return ""
	}()))
}

func (w *WebForms) ServiceWorkerDeleteDynamicCache(path ...string) {
	if len(path) > 0 {
		w.add("wd", path[0])
		return
	}

	w.addName("wd")
}

func (w *WebForms) ServiceWorkerDynamicCacheTTLUpdate(path string, seconds ...any) {
	secondsValue := ""

	if len(seconds) > 0 {
		switch value := seconds[0].(type) {
		case string:
			secondsValue = value
		case int:
			if value > 0 {
				secondsValue = strconv.Itoa(value)
			}
		}
	}

	value := path

	if secondsValue != "" {
		value += GS + secondsValue
	}

	w.add("wt", value)
}

func (w *WebForms) ServiceWorkerRouteSet(path string, routeType string, cacheDynamic ...bool) {
	cacheValue := false

	if len(cacheDynamic) > 0 {
		cacheValue = cacheDynamic[0]
	}

	value := path + GS + routeType

	if cacheValue {
		value += GS + "1"
	}

	w.add("wr", value)
}

func (w *WebForms) ServiceWorkerRouteAlias(path string, to string) {
	w.add("wa", path+GS+to)
}

func (w *WebForms) ServiceWorkerDeleteRouteAlias(path ...string) {
	if len(path) > 0 {
		w.add("wC", path[0])
		return
	}

	w.add("wC", "")
}

// Delete All Route And Alias
func (w *WebForms) ServiceWorkerDeleteRoute(path ...string) {
	if len(path) > 0 {
		w.add("wD", path[0])
		return
	}

	w.addName("wD")
}

// SSE
func (w *WebForms) DisconnectSSE(path string) {
	w.add("Ds", path)
}

func (w *WebForms) DisconnectAllSSE() {
	w.addName("Ds")
}

// State
func (w *WebForms) AddState(path ...string) {
	pathValue := ""
	titleValue := ""

	if len(path) > 0 {
		pathValue = path[0]
	}

	if len(path) > 1 {
		titleValue = path[1]
	}

	w.add("AS", pathValue+GS+titleValue)
}

func (w *WebForms) SaveState(path ...string) {
	pathValue := ""
	titleValue := ""

	if len(path) > 0 {
		pathValue = path[0]
	}

	if len(path) > 1 {
		titleValue = path[1]
	}

	w.add("As", pathValue+GS+titleValue)
}

func (w *WebForms) LoadState(path string) {
	w.add("ls", path)
}

func (w *WebForms) DeleteState(path ...string) {
	if len(path) > 0 {
		w.add("DS", path[0])
		return
	}

	w.add("DS", "")
}

func (w *WebForms) DeleteAllState() {
	w.add("DS", "*")
}

func (w *WebForms) LockQueue(millisecond any) {
	switch value := millisecond.(type) {
	case string:
		w.add("lq", value)
	case int:
		w.add("lq", strconv.Itoa(value))
	}
}

// Cookie
func (w *WebForms) SetCookie(key string, value string, seconds any, path ...string) {
	secondsValue := ""

	switch value := seconds.(type) {
	case string:
		secondsValue = value
	case int:
		secondsValue = strconv.Itoa(value)
	}

	data := key + GS + value + GS + secondsValue

	if len(path) > 0 && path[0] != "" {
		data += GS + path[0]
	}

	w.add("sC", data)
}

// Save (Session Cache)
func (w *WebForms) SaveId(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gi"+inputPlace, k)
}

func (w *WebForms) SaveName(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gn"+inputPlace, k)
}

func (w *WebForms) SaveValue(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gv"+inputPlace, k)
}

func (w *WebForms) SaveValueLength(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ge"+inputPlace, k)
}

func (w *WebForms) SaveClass(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gc"+inputPlace, k)
}

func (w *WebForms) SaveStyle(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gs"+inputPlace, k)
}

func (w *WebForms) SaveTitle(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gl"+inputPlace, k)
}

func (w *WebForms) SaveLabel(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gA"+inputPlace, k)
}

func (w *WebForms) SaveText(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gt"+inputPlace, k)
}

func (w *WebForms) SaveOuterText(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@go"+inputPlace, k)
}

func (w *WebForms) SaveTextLength(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gg"+inputPlace, k)
}

func (w *WebForms) SaveAttribute(inputPlace string, attribute string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ga"+inputPlace, k+GS+attribute)
}

func (w *WebForms) SaveWidth(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gw"+inputPlace, k)
}

func (w *WebForms) SaveHeight(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gh"+inputPlace, k)
}

func (w *WebForms) SaveReadOnly(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gr"+inputPlace, k)
}

func (w *WebForms) SaveSelectedIndex(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gx"+inputPlace, k)
}

func (w *WebForms) SaveTextAlign(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gT"+inputPlace, k)
}

func (w *WebForms) SaveNodeLength(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gL"+inputPlace, k)
}

func (w *WebForms) SaveVisible(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gV"+inputPlace, k)
}

func (w *WebForms) SaveUrl(url string, fetchScript ...bool) {
	fs := false
	key := "."
	if len(fetchScript) > 0 {
		fs = fetchScript[0]
	}
	if len(fetchScript) > 1 {
		key = fmt.Sprint(fetchScript[1])
	}
	w.add("@gu", key+GS+url+func() string {
		if fs {
			return GS + "1"
		}
		return ""
	}())
}

func (w *WebForms) SaveIndex(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@gI"+inputPlace, k)
}

func (w *WebForms) RemoveSave(cacheKey string) {
	w.add("rs", cacheKey)
}

func (w *WebForms) RemoveAllSave() {
	w.add("rs", "*")
}

// Calling the SetSave Method Causes Action Control Requests Triggered by Events Using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send Event, to be Temporarily Saved on the Active Page, so the Request will not be Sent to the Server Again.
func (w *WebForms) SetSave() {
	w.add("cs", "*")
}

func (w *WebForms) AddSaveValue(cacheKey string, value string) {
	w.add("SA", cacheKey+GS+strings.ReplaceAll(value, "\n", "$[ln];"))
}

func (w *WebForms) InsertSaveValue(cacheKey string, value string) {
	w.add("SI", cacheKey+GS+strings.ReplaceAll(value, "\n", "$[ln];"))
}

func (w *WebForms) AppendSaveValue(cacheKey string, value string) {
	w.add("SP", cacheKey+GS+strings.ReplaceAll(value, "\n", "$[ln];"))
}

func (w *WebForms) ReplaceSaveValue(cacheKey string, searchValue string, value string) {
	w.add("SR",
		cacheKey+GS+
			strings.ReplaceAll(value, "\n", "$[ln];")+GS+
			strings.ReplaceAll(searchValue, "\n", "$[ln];"))
}

// Is Regex Replace
func (w *WebForms) SetFormatSaveValue(cacheKey string, regex string, replacement string) {
	w.add("SF", cacheKey+GS+regex+GS+strings.ReplaceAll(replacement, "\n", "$[ln];"))
}

// Operator: +, -, *, /, %, //, **
func (w *WebForms) SetArithmeticSaveValue(cacheKey string, operator string, value any) {
	var strValue string
	switch v := value.(type) {
	case string:
		strValue = v
	case int:
		strValue = strconv.Itoa(v)
	}
	w.add("SM", cacheKey+GS+operator+GS+strValue)
}

// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
func (w *WebForms) SetTextOperationSaveValue(cacheKey string, operation string, value1 any, value2 any) {
	var v1, v2 string
	switch v := value1.(type) {
	case string:
		v1 = v
	case int:
		v1 = strconv.Itoa(v)
	}
	switch v := value2.(type) {
	case string:
		v2 = v
	case int:
		v2 = strconv.Itoa(v)
	}
	w.add("ST", cacheKey+GS+operation+GS+v1+GS+v2)
}

// Cache
func (w *WebForms) CacheId(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ci"+inputPlace, k)
}

func (w *WebForms) CacheName(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cn"+inputPlace, k)
}

func (w *WebForms) CacheValue(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cv"+inputPlace, k)
}

func (w *WebForms) CacheValueLength(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ce"+inputPlace, k)
}

func (w *WebForms) CacheClass(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cc"+inputPlace, k)
}

func (w *WebForms) CacheStyle(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cs"+inputPlace, k)
}

func (w *WebForms) CacheTitle(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cl"+inputPlace, k)
}

func (w *WebForms) CacheLabel(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cA"+inputPlace, k)
}

func (w *WebForms) CacheText(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ct"+inputPlace, k)
}

func (w *WebForms) CacheOuterText(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@co"+inputPlace, k)
}

func (w *WebForms) CacheTextLength(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cg"+inputPlace, k)
}

func (w *WebForms) CacheAttribute(inputPlace string, attribute string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ca"+inputPlace, k+GS+attribute)
}

func (w *WebForms) CacheWidth(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cw"+inputPlace, k)
}

func (w *WebForms) CacheHeight(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@ch"+inputPlace, k)
}

func (w *WebForms) CacheReadOnly(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cr"+inputPlace, k)
}

func (w *WebForms) CacheSelectedIndex(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cx"+inputPlace, k)
}

func (w *WebForms) CacheTextAlign(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cT"+inputPlace, k)
}

func (w *WebForms) CacheNodeLength(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cL"+inputPlace, k)
}

func (w *WebForms) CacheVisible(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cV"+inputPlace, k)
}

func (w *WebForms) CacheUrl(url string, fetchScript ...any) {
	fs := false
	key := "."
	if len(fetchScript) > 0 {
		fs, _ = fetchScript[0].(bool)
	}
	if len(fetchScript) > 1 {
		key = fmt.Sprint(fetchScript[1])
	}
	w.add("@cu", key+GS+url+func() string {
		if fs {
			return GS + "1"
		}
		return ""
	}())
}

func (w *WebForms) CacheIndex(inputPlace string, key ...string) {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}
	w.add("@cI"+inputPlace, k)
}

func (w *WebForms) RemoveCache(cacheKey string) {
	w.add("rd", cacheKey)
}

func (w *WebForms) RemoveAllCache() {
	w.add("rd", "*")
}

// Calling the SetCache Method Causes Action Control Requests Triggered by events using the GET, POST, PUT, PATCH, DELETE, and OPTIONS Methods, as well as Requests Triggered by the Send event, to be Cached, so the Request will not be Sent to the Server Again.
func (w *WebForms) SetCache(second ...any) {
	if len(second) == 0 {
		w.add("cd", "*")
		return
	}

	switch value := second[0].(type) {
	case int:
		w.add("cd", strconv.Itoa(value))
	case string:
		w.add("cd", value)
	}
}

func (w *WebForms) AddCacheValue(cacheKey string, value string) {
	w.add("CA", cacheKey+GS+strings.ReplaceAll(value, "\n", "$[ln];"))
}

func (w *WebForms) InsertCacheValue(cacheKey string, value string) {
	w.add("CI", cacheKey+GS+strings.ReplaceAll(value, "\n", "$[ln];"))
}

func (w *WebForms) AppendCacheValue(cacheKey string, value string) {
	w.add("CP", cacheKey+GS+strings.ReplaceAll(value, "\n", "$[ln];"))
}

func (w *WebForms) ReplaceCacheValue(cacheKey string, searchValue string, value string) {
	w.add("CR",
		cacheKey+GS+
			strings.ReplaceAll(value, "\n", "$[ln];")+GS+
			strings.ReplaceAll(searchValue, "\n", "$[ln];"))
}

// Is Regex Replace
func (w *WebForms) SetFormatCacheValue(cacheKey string, regex string, replacement string) {
	w.add("CF", cacheKey+GS+regex+GS+strings.ReplaceAll(replacement, "\n", "$[ln];"))
}

// Operator: +, -, *, /, %, //, **
func (w *WebForms) SetArithmeticCacheValue(cacheKey string, operator string, value any) {
	var strValue string
	switch v := value.(type) {
	case string:
		strValue = v
	case int:
		strValue = strconv.Itoa(v)
	}
	w.add("CM", cacheKey+GS+operator+GS+strValue)
}

// Operation: textafter, textafterlast, textbefore, textbeforelast, substring, remove
func (w *WebForms) SetTextOperationCacheValue(cacheKey string, operation string, value1 any, value2 any) {
	var v1, v2 string
	switch v := value1.(type) {
	case string:
		v1 = v
	case int:
		v1 = strconv.Itoa(v)
	}
	switch v := value2.(type) {
	case string:
		v2 = v
	case int:
		v2 = strconv.Itoa(v)
	}
	w.add("CT", cacheKey+GS+operation+GS+v1+GS+v2)
}

// Call
func (w *WebForms) LoadUrl(inputPlace string, url string) {
	w.add("lu"+inputPlace, url)
}

func (w *WebForms) RunActionControls(actionControls string, withoutWebFormsSection ...any) {
	withoutSection := true
	index := ""
	useCurrentEvent := true

	if len(withoutWebFormsSection) > 0 {
		withoutSection, _ = withoutWebFormsSection[0].(bool)
	}
	if len(withoutWebFormsSection) > 1 && withoutWebFormsSection[1] != nil {
		index = fmt.Sprint(withoutWebFormsSection[1])
	}
	if len(withoutWebFormsSection) > 2 {
		useCurrentEvent, _ = withoutWebFormsSection[2].(bool)
	}

	w.add("lA",
		boolString(useCurrentEvent)+GS+
			boolString(withoutSection)+GS+
			index+GS+actionControls)
}

func (w *WebForms) CallScript(scriptText string) {
	w.add("_", strings.ReplaceAll(scriptText, "\n", "$[ln];"))
}

func (w *WebForms) CallMethod(methodName string, args ...any) {
	argsJoin := ""

	if len(args) > 0 {
		argsJoin = GS + "[" + joinUS(args)
	}

	w.add("lm", methodName+argsJoin)
}

func (w *WebForms) CallModuleMethod(methodName string, args ...any) {
	argsJoin := ""

	if len(args) > 0 {
		argsJoin = GS + "[" + joinUS(args)
	}

	w.add("lM", methodName+argsJoin)
}

func (w *WebForms) CallPostBack(formInputPlace string, outputPlace ...string) {
	output := ""
	if len(outputPlace) > 0 && outputPlace[0] != "" {
		output = GS + outputPlace[0]
	}
	w.add("Lp", "1"+GS+formInputPlace+output)
}

func (w *WebForms) CallCommentBack(index ...any) {
	i := ""
	inputPlace := ""
	useCurrentEvent := true

	if len(index) > 0 && index[0] != nil {
		i = fmt.Sprint(index[0])
	}
	if len(index) > 1 && index[1] != nil {
		inputPlace = fmt.Sprint(index[1])
	}
	if len(index) > 2 {
		useCurrentEvent, _ = index[2].(bool)
	}

	w.add("LC", boolString(useCurrentEvent)+GS+i+GS+inputPlace)
}

func (w *WebForms) CallWasmBack(wasmLanguage string, wasmUrl string, methodName string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		if args[len(args)-1] != nil {
			if value, ok := args[len(args)-1].(bool); ok {
				useCurrentEvent = value
				args = args[:len(args)-1]
			}
		}
	}

	if len(args) > 0 {
		if value, ok := args[len(args)-1].(string); ok {
			outputPlace = value
			args = args[:len(args)-1]
		}
	}

	argsJoin := ""
	if len(args) > 0 {
		argsJoin = "[" + joinUS(args)
	}

	w.add("Ly",
		boolString(useCurrentEvent)+GS+
			wasmLanguage+GS+
			wasmUrl+GS+
			methodName+GS+
			argsJoin+GS+outputPlace)
}

func (w *WebForms) CallWebSocketBack(path string, useCurrentEvent ...bool) {
	use := true
	if len(useCurrentEvent) > 0 {
		use = useCurrentEvent[0]
	}
	w.add("Lw", boolString(use)+GS+path)
}

func (w *WebForms) CallSSEBack(path string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true
	shouldReconnect := true
	reconnectTryTimeout := "3000"

	if len(args) > 0 && args[0] != nil {
		outputPlace = fmt.Sprint(args[0])
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}
	if len(args) > 2 {
		shouldReconnect, _ = args[2].(bool)
	}
	if len(args) > 3 {
		reconnectTryTimeout = fmt.Sprint(args[3])
	}

	w.add("Ls",
		boolString(useCurrentEvent)+GS+
			path+GS+
			boolString(shouldReconnect)+GS+
			reconnectTryTimeout+
			func() string {
				if outputPlace != "" {
					return GS + outputPlace
				}
				return ""
			}())
}

func (w *WebForms) CallFront(modulePath string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		if value, ok := args[0].(string); ok {
			outputPlace = value
		}
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	callArgs := []any{}
	if len(args) > 2 {
		callArgs = args[2:]
	}

	argsJoin := ""
	if len(callArgs) > 0 {
		argsJoin = GS + "[" + joinUS(callArgs)
	}

	w.add("Lj",
		boolString(useCurrentEvent)+GS+
			modulePath+GS+
			outputPlace+argsJoin)
}

func (w *WebForms) CallGetBack(path string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		outputPlace, _ = args[0].(string)
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	w.add("Lg", boolString(useCurrentEvent)+GS+path+
		func() string {
			if outputPlace != "" {
				return GS + outputPlace
			}
			return ""
		}())
}

func (w *WebForms) CallPutBack(path string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		outputPlace, _ = args[0].(string)
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	w.add("Lt", boolString(useCurrentEvent)+GS+path+
		func() string {
			if outputPlace != "" {
				return GS + outputPlace
			}
			return ""
		}())
}

func (w *WebForms) CallPatchBack(path string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		outputPlace, _ = args[0].(string)
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	w.add("LP", boolString(useCurrentEvent)+GS+path+
		func() string {
			if outputPlace != "" {
				return GS + outputPlace
			}
			return ""
		}())
}

func (w *WebForms) CallDeleteBack(path string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		outputPlace, _ = args[0].(string)
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	w.add("Ld", boolString(useCurrentEvent)+GS+path+
		func() string {
			if outputPlace != "" {
				return GS + outputPlace
			}
			return ""
		}())
}

func (w *WebForms) CallHeadBack(path string, useCurrentEvent ...bool) {
	use := true
	if len(useCurrentEvent) > 0 {
		use = useCurrentEvent[0]
	}
	w.add("Lh", boolString(use)+GS+path)
}

func (w *WebForms) CallOptionsBack(path string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		outputPlace, _ = args[0].(string)
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	w.add("Lo", boolString(useCurrentEvent)+GS+path+
		func() string {
			if outputPlace != "" {
				return GS + outputPlace
			}
			return ""
		}())
}

func (w *WebForms) CallSendBack(path string, method string, isMultiPart bool, contentType string, data string, args ...any) {
	outputPlace := ""
	useCurrentEvent := true

	if len(args) > 0 {
		outputPlace, _ = args[0].(string)
	}
	if len(args) > 1 {
		useCurrentEvent, _ = args[1].(bool)
	}

	w.add("LS",
		boolString(useCurrentEvent)+GS+
			path+GS+
			method+GS+
			boolString(isMultiPart)+GS+
			contentType+GS+
			strings.ReplaceAll(data, "\n", "$[ln];")+
			func() string {
				if outputPlace != "" {
					return GS + outputPlace
				}
				return ""
			}())
}

// Update
func (w *WebForms) Increase(inputPlace string, value float32) {
	w.add("gt"+inputPlace, "i"+GS+strconv.FormatFloat(float64(value), 'f', -1, 32))
}

func (w *WebForms) Decrease(inputPlace string, value float32) {
	w.add("gt"+inputPlace, "i"+GS+strconv.FormatFloat(float64(value*-1), 'f', -1, 32))
}

// If You Don't Use Deep Mode, any Tags Inside the Current Tag Will Simply Be Treated as Strings. Deep Mode Does not Remove Inner Elements.
func (w *WebForms) Replace(inputPlace string, value string, newValue string, args ...bool) {
	alsoStartTag := false
	deep := true

	if len(args) > 0 {
		alsoStartTag = args[0]
	}
	if len(args) > 1 {
		deep = args[1]
	}

	w.add("gt"+inputPlace,
		"r"+GS+value+GS+newValue+GS+
			boolString(alsoStartTag)+GS+boolString(deep))
}

// HTML Converts Attribute Names to Lowercase, so they Need to Be Written in Lowercase.
func (w *WebForms) ReplaceStartTag(inputPlace string, value string, newValue string) {
	w.add("gt"+inputPlace, "s"+GS+value+GS+newValue)
}

// Pre Runner
func (w *WebForms) AssignDelay(milliSecond int, index ...int) {
	i := -1
	if len(index) > 0 {
		i = index[0]
	}

	currentLine := w.getLineByIndex(i)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	newName := ":" + strconv.Itoa(milliSecond) + ")" + parts[0]
	newValue := ""

	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(i, newName, newValue)
}

func (w *WebForms) AssignDelayChange(milliSecond int, index ...int) {
	i := -1
	if len(index) > 0 {
		i = index[0]
	}

	currentLine := w.getLineByIndex(i)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	currentName := parts[0]

	if strings.HasPrefix(currentName, ":") && strings.Contains(currentName, ")") {
		closingBracket := strings.Index(currentName, ")")
		currentName = currentName[closingBracket+1:]
	}

	newName := ":" + strconv.Itoa(milliSecond) + ")" + currentName
	newValue := ""

	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(i, newName, newValue)
}

func (w *WebForms) AssignInterval(milliSecond int, id string, index ...int) {
	i := -1
	if len(index) > 0 {
		i = index[0]
	}

	currentLine := w.getLineByIndex(i)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	newName := "(" + strconv.Itoa(milliSecond)
	if id != "" {
		newName += "|" + id
	}
	newName += ")" + parts[0]

	newValue := ""
	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(i, newName, newValue)
}

func (w *WebForms) AssignIntervalChange(milliSecond int, id string, index ...int) {
	i := -1
	if len(index) > 0 {
		i = index[0]
	}

	currentLine := w.getLineByIndex(i)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	currentName := parts[0]

	if strings.HasPrefix(currentName, "(") && strings.Contains(currentName, ")") {
		closingBracket := strings.Index(currentName, ")")
		currentName = currentName[closingBracket+1:]
	}

	newName := "(" + strconv.Itoa(milliSecond)
	if id != "" {
		newName += "|" + id
	}
	newName += ")" + currentName

	newValue := ""
	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(i, newName, newValue)
}

func (w *WebForms) DeleteInterval(id string) {
	w.add("Di", id)
}

func (w *WebForms) AssignRepeat(count int, index ...int) {
	i := -1
	if len(index) > 0 {
		i = index[0]
	}

	currentLine := w.getLineByIndex(i)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	newName := "," + strconv.Itoa(count) + ")" + parts[0]
	newValue := ""

	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(i, newName, newValue)
}

func (w *WebForms) AssignRepeatChange(count int, index ...int) {
	i := -1
	if len(index) > 0 {
		i = index[0]
	}

	currentLine := w.getLineByIndex(i)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	currentName := parts[0]

	if strings.HasPrefix(currentName, ",") && strings.Contains(currentName, ")") {
		closingBracket := strings.Index(currentName, ")")
		currentName = currentName[closingBracket+1:]
	}

	newName := "," + strconv.Itoa(count) + ")" + currentName
	newValue := ""

	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(i, newName, newValue)
}

// Index
func (w *WebForms) StartIndex(name ...string) {
	n := ""
	if len(name) > 0 {
		n = name[0]
	}
	w.add("#", n)
}

// This Index Is Automatically Run After Changing The Browser History (Back And Forward Buttons)
func (w *WebForms) StartState() {
	w.StartIndex("$")
}

func (w *WebForms) GoTo(line string, repeat ...string) {
	r := "1"

	if len(repeat) > 0 {
		r = repeat[0]
	}

	w.add("&", line+GS+r)
}

func (w *WebForms) GoToInt(line int, repeat ...int) {
	r := 1
	if len(repeat) > 0 {
		r = repeat[0]
	}
	w.GoTo(strconv.Itoa(line), strconv.Itoa(r))
}

func (w *WebForms) GoToIndex(index string, repeat ...int) {
	r := 1
	if len(repeat) > 0 {
		r = repeat[0]
	}
	w.add("&", "#"+index+GS+strconv.Itoa(r))
}

// Start
func (w *WebForms) StartTransientDOM(inputPlace string) {
	w.add("td", inputPlace)
}

func (w *WebForms) EndTransientDOM() {
	w.add("td", ";")
}

// Message
// Type: warning, problem, help, success, none
func (w *WebForms) Alert(text string, args ...string) {
	typ := "none"
	title := "Alert"
	okText := "OK"

	if len(args) > 0 {
		typ = args[0]
	}
	if len(args) > 1 {
		title = args[1]
	}
	if len(args) > 2 {
		okText = args[2]
	}

	if typ == "none" {
		typ = ""
	}
	if title == "Alert" {
		title = ""
	}
	if okText == "OK" {
		okText = ""
	}

	w.add("Al", text+GS+typ+GS+title+GS+okText)
}

func (w *WebForms) Message(text string, args ...any) {
	typ := "none"
	duration := "0"

	if len(args) > 0 {
		if value, ok := args[0].(string); ok {
			typ = value
		} else {
			duration = fmt.Sprint(args[0])
		}
	}

	if len(args) > 1 {
		duration = fmt.Sprint(args[1])
	}

	if typ == "none" {
		typ = ""
	}
	if duration == "0" {
		duration = ""
	}

	w.add("me", text+GS+typ+GS+duration)
}

// Type: log, info, warn, error, debug, trace, group, groupend, table
func (w *WebForms) ConsoleMessage(text string, typ ...string) {
	t := "log"
	if len(typ) > 0 {
		t = typ[0]
	}

	value := strings.ReplaceAll(text, "\n", "$[ln];")
	if t != "log" {
		value += GS + t
	}

	w.add("mc", value)
}

func (w *WebForms) ConsoleMessageAssert(text string, condition string) {
	w.add("ma", strings.ReplaceAll(text, "\n", "$[ln];")+GS+condition)
}

// Enable
//Calling The EnableWebSocket Or EnableWebSocketOnce Or AddWebSocket Methods Will Cause Any Subsequent Requests (Under WebForms Core Technology) To Operate Under The WebSocket Protocol.
func (w *WebForms) EnableWebSocket(enable ...bool) {
	e := true
	if len(enable) > 0 {
		e = enable[0]
	}
	w.add("ew", boolString(e))
}

func (w *WebForms) EnableWebSocketOnce() {
	w.add("ew", "$")
}

func (w *WebForms) AddWebSocket(path string) {
	w.add("aw"+path, "")
}

// Disconnected WebSocket
func (w *WebForms) DeleteWebSocket(path string) {
	w.add("dw"+path, "")
}

// Use
// InputPlace Using Only For form Element
func (w *WebForms) UseWebSocket(inputPlace string) {
	w.add("uw"+inputPlace, "")
}

func (w *WebForms) UseOnlyChangeUpdate(inputPlace string) {
	w.add("uo"+inputPlace, "")
}

// Condition And Loop
// Condition And Loop Supports Brackets and Then
// Type: warning, problem, help, success, none
// Interval: Value 0 is Await (if is not True, all Next Action Controls Waiting for it), Value -1 is Sync Check Once (is Support Bracket or Next Action Control), Value > 0 is Async and is Wait Based on Time Repetition Until it Becomes True (Is Support Bracket or Next Action Control, but is not Support Else).
// Nested Conditions and Nested Loops are Possible.
func (w *WebForms) ConfirmIsTrueAccept(text string, typ string, title string, okText string, cancelText string, interval int) *WebForms {
	if text == "" {
		text = "Are you sure you want to proceed?"
	}
	if typ == "" {
		typ = "none"
	}
	if title == "" {
		title = "Confirm"
	}
	if okText == "" {
		okText = "OK"
	}
	if cancelText == "" {
		cancelText = "Cancel"
	}
	if interval == 0 {
		interval = 100
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"ct",
		func() string {
			if text == "Are you sure you want to proceed?" {
				return ""
			}
			return text
		}()+GS+
			func() string {
				if typ == "none" {
					return ""
				}
				return typ
			}()+GS+
			func() string {
				if title == "Confirm" {
					return ""
				}
				return title
			}()+GS+
			func() string {
				if okText == "OK" {
					return ""
				}
				return okText
			}()+GS+
			func() string {
				if cancelText == "Cancel" {
					return ""
				}
				return cancelText
			}(),
	)

	return w
}

func (w *WebForms) ConfirmIsFalseAccept(text string, typ string, title string, okText string, cancelText string, interval int) *WebForms {
	if text == "" {
		text = "Are you sure you want to proceed?"
	}
	if typ == "" {
		typ = "none"
	}
	if title == "" {
		title = "Confirm"
	}
	if okText == "" {
		okText = "OK"
	}
	if cancelText == "" {
		cancelText = "Cancel"
	}
	if interval == 0 {
		interval = 100
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"cf",
		func() string {
			if text == "Are you sure you want to proceed?" {
				return ""
			}
			return text
		}()+GS+
			func() string {
				if typ == "none" {
					return ""
				}
				return typ
			}()+GS+
			func() string {
				if title == "Confirm" {
					return ""
				}
				return title
			}()+GS+
			func() string {
				if okText == "OK" {
					return ""
				}
				return okText
			}()+GS+
			func() string {
				if cancelText == "Cancel" {
					return ""
				}
				return cancelText
			}(),
	)

	return w
}

func (w *WebForms) IsGreaterThan(firstValue string, secondValue string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"gt",
		firstValue+GS+secondValue,
	)

	return w
}

func (w *WebForms) IsLessThan(firstValue string, secondValue string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"lt",
		firstValue+GS+secondValue,
	)

	return w
}

func (w *WebForms) IsEqualTo(firstValue string, secondValue string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"et",
		firstValue+GS+secondValue,
	)

	return w
}

func (w *WebForms) IsNotEqualTo(firstValue string, secondValue string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"Nt",
		firstValue+GS+secondValue,
	)

	return w
}

func (w *WebForms) Exist(value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"ex",
		value,
	)

	return w
}

func (w *WebForms) NotExist(value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"nx",
		value,
	)

	return w
}

func (w *WebForms) IsTrue(value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"tr",
		value,
	)

	return w
}

func (w *WebForms) IsFalse(value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"fa",
		value,
	)

	return w
}

func (w *WebForms) IsMatchMedia(value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"mm",
		value,
	)

	return w
}

func (w *WebForms) IsNotMatchMedia(value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"nm",
		value,
	)

	return w
}

func (w *WebForms) Include(text string, value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"In",
		value+GS+text,
	)

	return w
}

func (w *WebForms) NotInclude(text string, value string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"Nn",
		value+GS+text,
	)

	return w
}

func (w *WebForms) ElementExists(inputPlace string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"eE",
		inputPlace,
	)

	return w
}

func (w *WebForms) ElementNotExists(inputPlace string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"nE",
		inputPlace,
	)

	return w
}

func (w *WebForms) IsRegexMatch(value string, pattern string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"re",
		value+GS+pattern,
	)

	return w
}

func (w *WebForms) IsRegexNotMatch(value string, pattern string, interval int) *WebForms {
	if interval == 0 {
		interval = -1
	}

	w.add(
		func() string {
			if interval >= 0 {
				return "{(" + strconv.Itoa(interval) + ")"
			}
			return "{"
		}()+"rn",
		value+GS+pattern,
	)

	return w
}

// In: Everything Becomes A JSON List.
// Key: Creates A Temporary Data In The Browser IndexedDB.
// Key + "i" Creates A Temporary Data To Maintain The Loop Counter In The Browser IndexedDB.
func (w *WebForms) ForEach(path string, in string, key string) *WebForms {
	if key == "" {
		key = "."
	}

	w.add("{fe", path+GS+in+GS+key)

	return w
}

func (w *WebForms) Break() {
	w.add(";")
}

func (w *WebForms) Else() *WebForms {
	w.add("}e")
	return w
}

func (w *WebForms) StartBracket() {
	w.add("{")
}

func (w *WebForms) EndBracket() {
	w.add("}")
}

// High-Level Methods
// Used Then In Condition And Loop Methods
func (w *WebForms) Then(newForm *WebForms) *WebForms {
	var data string

	if newForm != nil {
		data = newForm.GetWebFormsData()
	}

	if data != "" {
		if strings.Contains(data, "\n") {
			newForm.addToUp("{")
			newForm.add("}")
		}
	}

	w.AppendForm(newForm)
	return w
}

func (w *WebForms) ThenConfigure(configure func(*WebForms)) *WebForms {
	newForm := New()
	configure(newForm)

	var data string

	if newForm != nil {
		data = newForm.GetWebFormsData()
	}

	if data != "" {
		if strings.Contains(data, "\n") {
			newForm.addToUp("{")
			newForm.add("}")
		}
	}

	w.AppendForm(newForm)
	return w
}

func (w *WebForms) Repeat(newForm *WebForms, repeat int) *WebForms {
	if newForm == nil {
		return w
	}

	bodyData := newForm.GetWebFormsData()

	if bodyData == "" {
		return w
	}

	startLine := -len(strings.Split(bodyData, "\n"))

	w.AppendForm(newForm)
	w.GoToInt(startLine, repeat-1)

	return w
}

func (w *WebForms) RepeatWithIndex(newForm *WebForms, repeat int, index string) *WebForms {
	if newForm == nil {
		return w
	}

	w.GoTo(index)
	w.StartIndex(index)

	bodyData := newForm.GetWebFormsData()

	if bodyData == "" {
		return w
	}

	w.AppendForm(newForm)

	if index == "" {
		indexNumber := -1

		for _, x := range strings.Split(w.GetWebFormsData(), "\n") {
			if strings.HasPrefix(x, "#") {
				indexNumber++
			}
		}

		w.GoTo(strconv.Itoa(indexNumber), strconv.Itoa(repeat-1))
	} else {
		w.GoTo(index, strconv.Itoa(repeat-1))
	}

	return w
}

func (w *WebForms) RepeatConfigure(configure func(*WebForms), repeat int) *WebForms {
	newForm := New()
	configure(newForm)

	return w.Repeat(newForm, repeat)
}

func (w *WebForms) RepeatConfigureWithIndex(configure func(*WebForms), repeat int, index string) *WebForms {
	newForm := New()
	configure(newForm)

	return w.RepeatWithIndex(newForm, repeat, index)
}

func (w *WebForms) Isole(newForm *WebForms, inputPlace string) *WebForms {
	if newForm == nil {
		return w
	}

	bodyData := newForm.GetWebFormsData()

	if bodyData == "" {
		return w
	}

	form := New()
	form.StartTransientDOM(inputPlace)

	w.AppendForm(form)
	newForm.EndTransientDOM()
	w.AppendForm(newForm)

	return w
}

func (w *WebForms) IsoleConfigure(configure func(*WebForms), inputPlace string) *WebForms {
	newForm := New()
	configure(newForm)
	return w.Isole(newForm, inputPlace)
}

// The Render Method is Sensitive to DOM Changes; It is Recommended to Assign a Stable ID to the Selected Element.
// This Method Utilizes the Transient DOM; Therefore, When Selecting InputPlaces, You Must Consider the Root of the Selected InputPlace Within the Method.
func (w *WebForms) Render(newForm *WebForms, inputPlace string, args ...any) *WebForms {
	if newForm == nil {
		return w
	}

	bodyData := newForm.GetWebFormsData()

	if bodyData == "" {
		return w
	}

	key := ""
	permanent := false

	if len(args) > 0 {
		if k, ok := args[0].(string); ok {
			key = k
		}
	}
	if len(args) > 1 {
		if p, ok := args[1].(bool); ok {
			permanent = p
		}
	}

	if key == "" {
		key = inputPlace
	}

	form := New()

	if permanent {
		form.Exist(Fetch.Cache(key), 0)
	} else {
		form.Exist(Fetch.Save(key), 0)
	}
	form.Rollback(inputPlace, key, permanent)
	form.Else()
	form.Snapshot(inputPlace, key, permanent)

	form.StartTransientDOM(inputPlace)

	w.AppendForm(form)
	newForm.EndTransientDOM()
	w.AppendForm(newForm)

	return w
}

func (w *WebForms) RenderConfigure(configure func(*WebForms), inputPlace string, args ...any) *WebForms {
	newForm := New()
	configure(newForm)
	return w.Render(newForm, inputPlace, args...)
}

// Async
// It Supports Brackets and Then
func (w *WebForms) Async() *WebForms {
	w.add("{(a)")
	return w
}

func (w *WebForms) Delay(milliseconds string) {
	w.add("De", milliseconds)
}

func (w *WebForms) DelayInt(milliseconds int) {
	w.Delay(strconv.Itoa(milliseconds))
}

// Option
func (w *WebForms) ChangeOption(name string, value string) {
	w.add("co", name+GS+value)
}

func (w *WebForms) ResetOption() {
	w.add("ro")
}

func (w *WebForms) ResetOptionName(name string) {
	w.add("ro", name)
}

// Format Storage
func (w *WebForms) CreateFormatStorage(key string, data string) {
	w.add(".C", key+GS+data)
}

func (w *WebForms) DeleteFormatStorage(key string) {
	w.add(".D", key)
}

func (w *WebForms) AddJSON(key string, path string, value string) {
	w.add(".a", key+GS+"j"+GS+value+GS+path)
}

// Name: For Support Attribute, Set Double At Sign (@@) Before Name.
func (w *WebForms) AddXML(key string, path string, name string, value string) {
	w.add(".a", key+GS+"x"+GS+name+GS+value+GS+path)
}

func (w *WebForms) AddINI(key string, path string, value string, isINILike bool) {
	w.add(".a", key+GS+"i"+GS+
		func() string {
			if isINILike {
				return "1"
			}
			return "0"
		}()+GS+value+GS+path,
	)
}

func (w *WebForms) AddTextLine(key string, line string, text string) {
	w.add(".a", key+GS+"t"+GS+text+GS+line)
}

func (w *WebForms) AddTextLineInt(key string, line int, text string) {
	w.AddTextLine(key, strconv.Itoa(line), text)
}

func (w *WebForms) AddVariable(key string, value string) {
	w.add(".a", key+GS+"v"+GS+value)
}

func (w *WebForms) UpdateJSON(key string, path string, value string) {
	w.add(".u", key+GS+"j"+GS+value+GS+path)
}

func (w *WebForms) UpdateXML(key string, path string, value string) {
	w.add(".u", key+GS+"x"+GS+value+GS+path)
}

func (w *WebForms) UpdateINI(key string, path string, value string, isINILike bool) {
	w.add(".u", key+GS+"i"+GS+
		func() string {
			if isINILike {
				return "1"
			}
			return "0"
		}()+GS+value+GS+path,
	)
}

func (w *WebForms) UpdateTexLine(key string, line string, text string) {
	w.add(".u", key+GS+"t"+GS+text+GS+line)
}

func (w *WebForms) UpdateTexLineInt(key string, line int, text string) {
	w.UpdateTexLine(key, strconv.Itoa(line), text)
}

func (w *WebForms) UpdateVariable(key string, value string) {
	w.add(".u", key+GS+"v"+GS+value)
}

func (w *WebForms) IncreaseVariable(key string, value string) {
	w.add(".i", key+GS+"v"+GS+value)
}

func (w *WebForms) IncreaseVariableInt(key string, value int) {
	w.IncreaseVariable(key, strconv.Itoa(value))
}

func (w *WebForms) DecreaseVariable(key string, value int) {
	w.IncreaseVariable(key, strconv.Itoa(value*-1))
}

func (w *WebForms) DeleteJSON(key string, path string) {
	w.add(".d", key+GS+"j"+GS+path)
}

func (w *WebForms) DeleteXML(key string, path string) {
	w.add(".d", key+GS+"x"+GS+path)
}

func (w *WebForms) DeleteINI(key string, path string, isINILike ...bool) {
	iniLike := false

	if len(isINILike) > 0 {
		iniLike = isINILike[0]
	}

	w.add(".d", key+GS+"i"+GS+boolString(iniLike)+GS+path)
}

func (w *WebForms) DeleteTextLine(key string, line string) {
	w.add(".d", key+GS+"t"+GS+line)
}

func (w *WebForms) DeleteTextLineInt(key string, line int) {
	w.DeleteTextLine(key, strconv.Itoa(line))
}

func (w *WebForms) DeleteVariable(key string) {
	w.add(".d", key+GS+"v")
}

// Template Engine
// Pattern Example: {{value}}, ((value)), *value*, $value;
func (w *WebForms) BindJSONToTemplate(inputPlace string, jsonText string, path string, pattern string, alsoStartTag bool) {
	w.add("Tj"+inputPlace, jsonText+GS+path+GS+pattern+GS+func() string {
		if alsoStartTag {
			return "1"
		}
		return "0"
	}())
}

// Because XML Elements Are Lowercased, Placeholders Must Use Lowercase Names.
func (w *WebForms) BindXMLToTemplate(inputPlace string, xmlText string, path string, pattern string, alsoStartTag bool) {
	w.add("Tx"+inputPlace, xmlText+GS+path+GS+pattern+GS+func() string {
		if alsoStartTag {
			return "1"
		}
		return "0"
	}())
}

func (w *WebForms) BindINIToTemplate(inputPlace string, iniText string, path string, pattern string, alsoStartTag bool) {
	w.add("Ti"+inputPlace, iniText+GS+path+GS+pattern+GS+func() string {
		if alsoStartTag {
			return "1"
		}
		return "0"
	}())
}

// Inject
// Need Add @: to First of String
func (w *WebForms) Inject(value string) string {
	return "$[" + value + "];"
}

// Action Control
func (w *WebForms) ReplaceActionControl(searchValue string, value string, addingToUp bool) {
	if addingToUp {
		w.addToUp("rE", searchValue+GS+value)
	} else {
		w.add("rE", searchValue+GS+value)
	}
}

func (w *WebForms) AssignReplace(searchValue string, value string, index int) {
	currentLine := w.getLineByIndex(index)
	if currentLine == "" {
		return
	}

	parts := strings.SplitN(currentLine, "=", 2)
	newName := ";" + searchValue + GS + value + GS + parts[0]
	newValue := ""
	if len(parts) > 1 {
		newValue = parts[1]
	}

	w.updateLineByIndex(index, newName, newValue)
}

// Hash And Checksum
func (w *WebForms) SetHash() {
	w.add("SH")
}

func (w *WebForms) SetChecksum() {
	w.add("CS")
}

func (w *WebForms) ChecksumCalculation(text string) string {
	sum := 0
	mod := 65536
	shift := 5

	for _, c := range text {
		sum = ((sum << shift) | (sum >> (16 - shift))) ^ int(c)
		sum %= mod
	}

	return strconv.Itoa(sum)
}

func (w *WebForms) GetChecksum() string {
	return w.ChecksumCalculation(w.GetWebFormsData())
}

// Get
func (w *WebForms) GetFormsActionData() string {
    if len(w.webFormsData) == 0 {
        return ""
    }

    return w.webFormsData
}

func (w *WebForms) Response() string {
	return "[web-forms]\n" + w.GetFormsActionData()
}

func (w *WebForms) GetFormsActionDataLineBreak() string {
	if len(w.webFormsData) == 0 {
		return ""
	}

	data := w.webFormsData
	processedData := strings.ReplaceAll(data, "\"", "$[dq];")
	return strings.ReplaceAll(processedData, "\n", "$[sln];")
}

// Export
func (w *WebForms) ExportToHtmlComment(addLine bool) string {
	response := strings.ReplaceAll(w.Response(), "--", "$[dd];")
	if response[len(response)-1] == '-' {
		response = response[:len(response)-1] + "$[da];"
	}

	if addLine {
		return "\n" + "<!--" + response + "-->"
	}
	return "<!--" + response + "-->"
}

// Using it for SSE Response
func (w *WebForms) ExportToLineBreak(src string) string {
	return "[web-forms]$[sln];" + w.GetFormsActionDataLineBreak()
}

func (w *WebForms) GetWebFormsData() string {
	return w.webFormsData
}

func (w *WebForms) AppendForm(form *WebForms) {
	if form == nil {
		return
	}

	otherData := form.GetWebFormsData()
	if otherData != "" {
		if len(w.webFormsData) > 0 {
			w.webFormsData += "\n"
		}
		w.webFormsData += otherData
	}
}

func (w *WebForms) Clean() {
	w.webFormsData = ""
}

type Security struct {
}

func (s *Security) SafeValue(value string) string {
	if len(value) < 1 {
		return value
	}

	if value[0] == '@' {
		value = "@" + value
	}

	value = strings.ReplaceAll(value, "\n", "$[ln];")
	value = strings.ReplaceAll(value, ",@", "$[co];@")
	value = strings.ReplaceAll(value, string(rune(28)), "\x00")
	value = strings.ReplaceAll(value, string(rune(29)), "\x00")
	value = strings.ReplaceAll(value, string(rune(30)), "\x00")
	value = strings.ReplaceAll(value, string(rune(31)), "\x00")

	return value
}

// WebForms Place Criteria (WPC) DSL
type inputPlace struct {
	Document          string
	Window            string
	Root              string
	HTML              string
	Head              string
	ScreenOrientation string
	All               string
	Parent            string
	Current           string
	Target            string
	Upper             string
}

var InputPlace = inputPlace{
	Document:          ",",
	Window:            "`",
	Root:              "~",
	HTML:              ".",
	Head:              "^",
	ScreenOrientation: "%",
	All:               "*",
	Parent:             "/",
	Current:            "$",
	Target:             "!",
	Upper:             "-",
}

// When Calling TransientDOM, Using Root will Result in the Selection of the Transient Tag.

func (inputPlace) Id(id string) string {
	return id
}

func (inputPlace) Name(name string) string {
	return "(" + name + ")"
}

func (inputPlace) NameIndex(name string, index int) string {
	return "(" + name + ")" + strconv.Itoa(index)
}

func (inputPlace) AllNames(name string) string {
	return "(" + name + ")*"
}

func (inputPlace) Tag(tag string) string {
	return "<" + tag + ">"
}

func (inputPlace) TagIndex(tag string, index int) string {
	return "<" + tag + ">" + strconv.Itoa(index)
}

func (inputPlace) AllTags(tag string) string {
	return "<" + tag + ">*"
}

func (inputPlace) Child() string {
	return "<>"
}

func (inputPlace) ChildIndex(index int) string {
	return "<>" + strconv.Itoa(index)
}

func (inputPlace) AllChild() string {
	return "<>*"
}

func (inputPlace) Class(class string) string {
	return "{" + class + "}"
}

func (inputPlace) ClassIndex(class string, index int) string {
	return "{" + class + "}" + strconv.Itoa(index)
}

func (inputPlace) AllClasses(class string) string {
	return "{" + class + "}*"
}

func (inputPlace) Attribute(name string) string {
	return "\"" + name + "\""
}

// Operator: '^', '$', '*', '~'
func (inputPlace) AttributeValue(name string, value string, operator ...rune) string {
	op := ""
	if len(operator) > 0 && operator[0] != '\x00' {
		op = string(operator[0])
	}

	return "\"" + name + op + "'" + value + "\""
}

func (inputPlace) AttributeIndex(name string, index int) string {
	return "\"" + name + "\"" + strconv.Itoa(index)
}

func (inputPlace) AttributeValueIndex(name string, value string, index int, operator ...rune) string {
	op := ""
	if len(operator) > 0 && operator[0] != '\x00' {
		op = string(operator[0])
	}

	return "\"" + name + op + "'" + value + "\"" + strconv.Itoa(index)
}

func (inputPlace) AllAttributes(name string) string {
	return "\"" + name + "\"*"
}

func (inputPlace) AllAttributesValue(name string, value string, operator ...rune) string {
	op := ""
	if len(operator) > 0 && operator[0] != '\x00' {
		op = string(operator[0])
	}

	return "\"" + name + op + "'" + value + "\"*"
}

func (inputPlace) Query(query string) string {
	query = strings.ReplaceAll(query, "=", "$[eq];")
	query = strings.ReplaceAll(query, "|", "$[vb];")
	query = strings.ReplaceAll(query, "?", "$[qu];")

	return "*" + query
}

func (inputPlace) QueryAll(query string) string {
	query = strings.ReplaceAll(query, "=", "$[eq];")
	query = strings.ReplaceAll(query, "|", "$[vb];")
	query = strings.ReplaceAll(query, "?", "$[qu];")

	return "[" + query + "]"
}

type OutputPlace struct {
	inputPlace
}

// Do not Add any Data Before or After it
type fetch struct {
	// Data
	DateYear         string
	DateMonth        string
	DateDay          string
	DateDate         string
	DateHours        string
	DateMinutes      string
	DateSeconds      string
	DateMilliseconds string

	// String
	Space  string
	AtSign string

	// Document
	TabIsActive string

	// Window
	Href         string
	PathName     string
	Hash         string
	Host         string
	HostName     string
	Port         string
	Origin       string
	GetSelection string
	ScrollX      string
	ScrollY      string

	// Navigator
	ClipboardText string
	GeoLatitude   string
	GeoLongitude  string
	Language      string
	IsOnLine      string
	UserAgent     string

	// Screen
	ScreenWidth            string
	ScreenHeight           string
	ScreenOrientationType  string
	ScreenOrientationAngle string

	// Performance
	TimeOrigin     string
	PerformanceNow string

	// Event
	Event          string
	EventSerialize string
	EventKey       string
	EventWhich     string
	EventClientX   string
	EventClientY   string
	EventPageX     string
	EventPageY     string
	EventOffsetX   string
	EventOffsetY   string
	EventDeltaY    string
}

const (
	FetchRS = string(rune(30))
	FetchUS = string(rune(31))
)

var Fetch = fetch{
	// Date
	DateYear:         "@dy",
	DateMonth:        "@dm",
	DateDay:          "@dd",
	DateDate:         "@dD",
	DateHours:        "@dh",
	DateMinutes:      "@di",
	DateSeconds:      "@ds",
	DateMilliseconds: "@dl",

	// String
	Space:  "@sp",
	AtSign: "@sa",

	// Document
	TabIsActive: "@da",

	// Window
	Href:         "@wf",
	PathName:     "@wP",
	Hash:         "@wh",
	Host:         "@wH",
	HostName:     "@wn",
	Port:         "@wT",
	Origin:       "@wo",
	GetSelection: "@ws",
	ScrollX:      "@wx",
	ScrollY:      "@wy",

	// Navigator
	ClipboardText: "@nC",
	GeoLatitude:   "@nW",
	GeoLongitude:  "@nO",
	Language:      "@nL",
	IsOnLine:      "@no",
	UserAgent:     "@na",

	// Screen
	ScreenWidth:            "@sw",
	ScreenHeight:           "@sh",
	ScreenOrientationType:  "@so",
	ScreenOrientationAngle: "@sr",

	// Performance
	TimeOrigin:     "@pt",
	PerformanceNow: "@pn",

	// Event
	Event:          "@EV",
	EventSerialize: "@Es",
	EventKey:       "@ek",
	EventWhich:     "@ew",
	EventClientX:   "@ex",
	EventClientY:   "@ey",
	EventPageX:     "@eX",
	EventPageY:     "@eY",
	EventOffsetX:   "@Ex",
	EventOffsetY:   "@Ey",
	EventDeltaY:    "@ed",
}

// Method
func (fetch) Random(maxValue int) string {
	return "@mr" + strconv.Itoa(maxValue)
}

func (fetch) RandomRange(minValue int, maxValue int) string {
	return "@mr" + strconv.Itoa(maxValue) + string(FetchRS) + strconv.Itoa(minValue)
}

func (fetch) SpaceToChar(text string, character string) string {
	return "@sc" + character + string(FetchRS) + text
}

func (fetch) EncodeURI(text string) string {
	return "@ue" + text
}

func (fetch) DecodeURI(text string) string {
	return "@ud" + text
}

func (fetch) Method(methodName string, args ...any) string {
	returnValue := "@cm" + methodName

	if args != nil {
		if len(args) > 0 {
			values := make([]string, len(args))
			for i, arg := range args {
				values[i] = fmt.Sprint(arg)
			}
			returnValue += string(FetchRS) + strings.Join(values, string(FetchUS))
		}
	}

	return returnValue
}

func (fetch) ModuleMethod(methodName string, args ...any) string {
	returnValue := "@cM" + methodName

	if args != nil {
		if len(args) > 0 {
			values := make([]string, len(args))
			for i, arg := range args {
				values[i] = fmt.Sprint(arg)
			}
			returnValue += string(FetchRS) + strings.Join(values, string(FetchUS))
		}
	}

	return returnValue
}

// MethodName: The Method Name May Need to Include the Class Name, Separated by a Period. Example: MyClassName.MyMethodName
func (fetch) WasmMethod(wasmLanguage string, wasmURL string, methodName string, args []any) string {
	returnValue := "@wA" + wasmLanguage + string(FetchRS) + wasmURL + string(FetchRS) + methodName

	if args != nil {
		if len(args) > 0 {
			values := make([]string, len(args))
			for i, arg := range args {
				values[i] = fmt.Sprint(arg)
			}
			returnValue += string(FetchRS) + strings.Join(values, string(FetchUS))
		}
	}

	return returnValue
}

func (fetch) Script(scriptText string) string {
	return "@_" + strings.ReplaceAll(scriptText, "\n", "$[ln];")
}

func (fetch) LoadURL(url string, fetchScript bool) string {
	if fetchScript {
		return "@lu" + url + string(FetchRS) + "1"
	}
	return "@lu" + url
}

func (fetch) LoadHTML(url string, fetchInputPlace string, fetchScript bool) string {
	result := "@lh" + url + string(FetchRS)
	if fetchScript {
		result += "1"
	} else {
		result += "0"
	}

	if fetchInputPlace != "" {
		result += string(FetchRS) + fetchInputPlace
	}

	return result
}

func (fetch) LoadLine(url string, line int) string {
	return "@ll" + url + string(FetchRS) + strconv.Itoa(line)
}

func (fetch) LoadINI(url string, name string, isINILike bool) string {
	result := "@li" + url + string(FetchRS) + name
	if isINILike {
		result += string(FetchRS) + "1"
	}
	return result
}

// Name: Name Or Nested Paths. Is Supprt Index (Student[8].Name). Nested Paths Index Starts At 0
func (fetch) LoadJSON(url string, name string) string {
	return "@lj" + url + string(FetchRS) + name
}

// Name: Name Or XPath; XPath Index Starts At 1
func (fetch) LoadXML(url string, name string) string {
	return "@lx" + url + string(FetchRS) + name
}

// MethodName: It's Check Function Or Variable
func (fetch) HasMethod(methodName string) string {
	return "@hm" + methodName
}

func (fetch) HasModuleMethod(methodName string) string {
	return "@hM" + methodName
}

// This Method Return True Or False If Key Pressed
// Modifier: Alt, AltGraph, Control, Meta, Shift, CapsLock, NumLock, ScrollLock
func (fetch) GetModifierState(modifier string) string {
	return "@ms" + modifier
}

// Math
func (fetch) Math(methodName string, args ...any) string {
	returnValue := "@M#" + methodName

	if args != nil {
		if len(args) > 0 {
			values := make([]string, len(args))
			for i, arg := range args {
				values[i] = fmt.Sprint(arg)
			}
			returnValue += string(FetchRS) + strings.Join(values, string(FetchUS))
		}
	}

	return returnValue
}

// Tag
func (fetch) GetId(inputPlace string) string {
	return "@$i" + inputPlace
}

func (fetch) GetName(inputPlace string) string {
	return "@$n" + inputPlace
}

func (fetch) GetValue(inputPlace string) string {
	return "@$v" + inputPlace
}

func (fetch) GetValueLength(inputPlace string) string {
	return "@$e" + inputPlace
}

func (fetch) GetClass(inputPlace string) string {
	return "@$c" + inputPlace
}

func (fetch) GetStyle(inputPlace string) string {
	return "@$s" + inputPlace
}

func (fetch) GetTitle(inputPlace string) string {
	return "@$l" + inputPlace
}

func (fetch) GetLabel(inputPlace string) string {
	return "@$A" + inputPlace
}

func (fetch) GetText(inputPlace string) string {
	return "@$t" + inputPlace
}

func (fetch) GetOuterText(inputPlace string) string {
	return "@$o" + inputPlace
}

func (fetch) GetTextLength(inputPlace string) string {
	return "@$g" + inputPlace
}

func (fetch) GetAttribute(inputPlace string, attribute string) string {
	return "@$a" + inputPlace + FetchRS + attribute
}

func (fetch) GetWidth(inputPlace string) string {
	return "@$w" + inputPlace
}

func (fetch) GetHeight(inputPlace string) string {
	return "@$h" + inputPlace
}

func (fetch) GetIsReadOnly(inputPlace string) string {
	return "@$r" + inputPlace
}

func (fetch) GetSelectedIndex(inputPlace string) string {
	return "@$x" + inputPlace
}

func (fetch) GetIndex(inputPlace string) string {
	return "@$I" + inputPlace
}

func (fetch) GetTextAlign(inputPlace string) string {
	return "@$T" + inputPlace
}

func (fetch) GetNodeLength(inputPlace string) string {
	return "@$L" + inputPlace
}

func (fetch) GetIsVisible(inputPlace string) string {
	return "@$V" + inputPlace
}

func (fetch) GetTagHash(inputPlace string) string {
	return "@$H" + inputPlace
}

// Save and Cache
func (fetch) HasHash(hash string) string {
	return "@HH" + hash
}

func (fetch) Cookie(key string) string {
	return "@co" + key
}

func (fetch) Save(key ...string) string {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}

	if len(key) > 1 {
		return "@cs" + k + FetchRS + key[1]
	}

	return "@cs" + k
}

func (fetch) SaveThenRemove(key string) string {
	return "@cl" + key
}

func (fetch) SaveLength(key ...string) string {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}

	return "@cg" + k
}

func (fetch) Cache(key ...string) string {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}

	if len(key) > 1 {
		return "@cd" + k + FetchRS + key[1]
	}

	return "@cd" + k
}

func (fetch) CacheThenRemove(key string) string {
	return "@ct" + key
}

func (fetch) CacheLength(key ...string) string {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}

	return "@cG" + k
}

func (fetch) SaveLine(key string, line ...int) string {
	k := "."
	if key != "" {
		k = key
	}

	l := 0
	if len(line) > 0 {
		l = line[0]
	}

	return "@lL" + k + "[" + strconv.Itoa(l)
}

func (fetch) SaveLineConsume(key ...string) string {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}

	return "@lL" + k
}

// INIKey: Only Direct Key is Supported
func (fetch) SaveINI(key string, iniKey string) string {
	return "@lI" + key + "[" + iniKey
}

func (fetch) CacheLine(key string, line ...int) string {
	k := "."
	if key != "" {
		k = key
	}

	l := 0
	if len(line) > 0 {
		l = line[0]
	}

	return "@dL" + k + "[" + strconv.Itoa(l)
}

func (fetch) CacheLineConsume(key ...string) string {
	k := "."
	if len(key) > 0 {
		k = key[0]
	}

	return "@dL" + k
}

// INIKey: Only Direct Key is Supported
func (fetch) CacheINI(key string, iniKey string) string {
	return "@dI" + key + "[" + iniKey
}

// Format Storage
func (fetch) FormatStore(key string) string {
	return "@fr" + key
}

func (fetch) FormatStoreByXMLQuery(key string, xpath string) string {
	return "@fx" + key + FetchRS + xpath
}

func (fetch) FormatStoreByJSONQuery(key string, query string) string {
	return "@fj" + key + FetchRS + query
}

func (fetch) FormatStoreByINI(key string, name string) string {
	return "@fi" + key + FetchRS + name
}

func (fetch) FormatStoreByText(key string, line int) string {
	return "@ft" + key + FetchRS + strconv.Itoa(line)
}

func (fetch) FormatStoreByVariable(key string) string {
	return "@fv" + key
}

// State
func (fetch) HasState(path string) string {
	return "@hs" + path
}

// SSE
func (fetch) SSEIsConnected(path string) string {
	return "@Sc" + path
}

// WebSockets
func (fetch) WebSocketsIsConnected(path ...string) string {
	p := ""
	if len(path) > 0 {
		p = path[0]
	}

	return "@Wc" + p
}

func (fetch) Query(name ...string) string {
	n := "*"
	if len(name) > 0 {
		n = name[0]
	}

	return "@wq" + n
}

func (fetch) Segment(index int) string {
	return "@wS" + strconv.Itoa(index)
}

// It Only Works when the String Starts with the Tilde Character (~). The Path is Also Separated by the Slash Character (/). #~/Segment1/Segment2/Segment3
func (fetch) HashSegment(index int) string {
	return "@wt" + strconv.Itoa(index)
}

type wasmLanguage struct {
	C              string
	CPP            string
	Rust           string
	CSharp         string
	GO             string
	JAVA           string
	AssemblyScript string
}

var WasmLanguage = wasmLanguage{
	C:              "c",
	CPP:            "c",
	Rust:           "rust",
	CSharp:         "csharp",
	GO:             "go",
	JAVA:            "java",
	AssemblyScript: "as",
}

type htmlEvent struct {
	OnAbort          string
	OnAfterPrint     string
	OnBeforePrint    string
	OnBeforeUnload   string
	OnBlur           string
	OnCanPlay        string
	OnCanPlayThrough string
	OnChange         string
	OnClick          string
	OnCopy           string
	OnCut            string
	OnDoubleClick    string
	OnDrag            string
	OnDragEnd        string
	OnDragEnter      string
	OnDragLeave      string
	OnDragOver       string
	OnDragStart      string
	OnDrop            string
	OnDurationChange string
	OnEnded          string
	OnError          string
	OnFocus          string
	OnFocusin        string
	OnFocusOut       string
	OnHashChange     string
	OnInput          string
	OnInvalid        string
	OnKeyDown        string
	OnKeyPress       string
	OnKeyUp          string
	OnLoad            string
	OnLoadedData      string
	OnLoadedMetaData  string
	OnLoadStart       string
	OnMouseDown       string
	OnMouseEnter      string
	OnMouseLeave      string
	OnMouseMove       string
	OnMouseOver       string
	OnMouseOut        string
	OnMouseUp         string
	OnOffline         string
	OnOnline          string
	OnPageHide        string
	OnPageShow        string
	OnPaste           string
	OnPause           string
	OnPlay            string
	OnPlaying         string
	OnProgress        string
	OnRateChange      string
	OnResize          string
	OnReset           string
	OnScroll          string
	OnSearch          string
	OnSeeked          string
	OnSeeking         string
	OnSelect          string
	OnStalled         string
	OnSubmit          string
	OnSuspend         string
	OnTimeUpdate      string
	OnToggle          string
	OnTouchCancel     string
	OnTouchend        string
	OnTouchMove       string
	OnTouchStart      string
	OnUnload          string
	OnVolumeChange    string
	OnWaiting         string
	OnWheel           string
}

var HtmlEvent = htmlEvent{
	OnAbort:          "onabort",
	OnAfterPrint:     "onafterprint",
	OnBeforePrint:    "onbeforeprint",
	OnBeforeUnload:   "onbeforeunload",
	OnBlur:           "onblur",
	OnCanPlay:        "oncanplay",
	OnCanPlayThrough: "oncanplaythrough",
	OnChange:         "onchange",
	OnClick:          "onclick",
	OnCopy:           "oncopy",
	OnCut:            "oncut",
	OnDoubleClick:    "ondblclick",
	OnDrag:            "ondrag",
	OnDragEnd:         "ondragend",
	OnDragEnter:       "ondragenter",
	OnDragLeave:       "ondragleave",
	OnDragOver:        "ondragover",
	OnDragStart:       "ondragstart",
	OnDrop:            "ondrop",
	OnDurationChange:  "ondurationchange",
	OnEnded:           "onended",
	OnError:           "onerror",
	OnFocus:           "onfocus",
	OnFocusin:         "onfocusin",
	OnFocusOut:        "onfocusout",
	OnHashChange:      "onhashchange",
	OnInput:           "oninput",
	OnInvalid:         "oninvalid",
	OnKeyDown:         "onkeydown",
	OnKeyPress:        "onkeypress",
	OnKeyUp:           "onkeyup",
	OnLoad:             "onload",
	OnLoadedData:      "onloadeddata",
	OnLoadedMetaData:  "onloadedmetadata",
	OnLoadStart:       "onloadstart",
	OnMouseDown:       "onmousedown",
	OnMouseEnter:      "onmouseenter",
	OnMouseLeave:      "onmouseleave",
	OnMouseMove:       "onmousemove",
	OnMouseOver:       "onmouseover",
	OnMouseOut:        "onmouseout",
	OnMouseUp:          "onmouseup",
	OnOffline:         "onoffline",
	OnOnline:          "ononline",
	OnPageHide:         "onpagehide",
	OnPageShow:         "onpageshow",
	OnPaste:            "onpaste",
	OnPause:            "onpause",
	OnPlay:             "onplay",
	OnPlaying:          "onplaying",
	OnProgress:         "onprogress",
	OnRateChange:       "onratechange",
	OnResize:           "onresize",
	OnReset:            "onreset",
	OnScroll:           "onscroll",
	OnSearch:           "onsearch",
	OnSeeked:           "onseeked",
	OnSeeking:          "onseeking",
	OnSelect:           "onselect",
	OnStalled:          "onstalled",
	OnSubmit:           "onsubmit",
	OnSuspend:          "onsuspend",
	OnTimeUpdate:       "ontimeupdate",
	OnToggle:            "ontoggle",
	OnTouchCancel:      "ontouchcancel",
	OnTouchend:         "ontouchend",
	OnTouchMove:        "ontouchmove",
	OnTouchStart:       "ontouchstart",
	OnUnload:           "onunload",
	OnVolumeChange:     "onvolumechange",
	OnWaiting:          "onwaiting",
	OnWheel:            "onwheel",
}

type htmlEventListener struct {
	Paste         string
	Pause         string
	Play          string
	Playing       string
	Progress      string
	RateChange    string
	Resize        string
	Reset         string
	Scroll        string
	Search        string
	Seeked        string
	Seeking       string
	Select        string
	Stalled       string
	Submit        string
	Suspend       string
	TimeUpdate    string
	Toggle        string
	TouchCancel   string
	Touchend      string
	TouchMove     string
	TouchStart    string
	Unload        string
	VolumeChange  string
	Waiting       string
	Wheel         string

	AnimationEnd       string
	AnimationIteration string
	AnimationStart     string
	ContextMenu        string
	FullScreenChange   string
	FullScreenError    string
	PopState           string
	TransitionEnd      string
	Storage            string
	
	ScrollBottom            string
	ElementReached            string
}

var HtmlEventListener = htmlEventListener{
	Paste:              "paste",
	Pause:              "pause",
	Play:               "play",
	Playing:            "playing",
	Progress:           "progress",
	RateChange:         "ratechange",
	Resize:             "resize",
	Reset:              "reset",
	Scroll:             "scroll",
	Search:             "search",
	Seeked:             "seeked",
	Seeking:            "seeking",
	Select:             "select",
	Stalled:            "stalled",
	Submit:             "submit",
	Suspend:            "suspend",
	TimeUpdate:         "timeupdate",
	Toggle:             "toggle",
	TouchCancel:        "touchcancel",
	Touchend:            "touchend",
	TouchMove:           "touchmove",
	TouchStart:          "touchstart",
	Unload:              "unload",
	VolumeChange:        "volumechange",
	Waiting:             "waiting",
	Wheel:               "wheel",

	AnimationEnd:       "animationend",
	AnimationIteration: "animationiteration",
	AnimationStart:     "animationstart",
	ContextMenu:        "contextmenu",
	FullScreenChange:   "fullscreenchange",
	FullScreenError:    "fullscreenerror",
	PopState:           "popstate",
	TransitionEnd:      "transitionend",
	Storage:            "storage",
	
	// Custom
	ScrollBottom:		"scrollbottom",		// Need Call EnableScrollBottomEvent Method Before
	ElementReached: 	"elementreached",	// Need Call EnableReachedElementEvent Method Before
}




func Child(text string, value string) string {
	if len(text) < 1 {
		return value
	}

	return text + "|" + value
}

func Parent(text string) string {
	if len(text) < 1 {
		return text
	}

	if strings.HasSuffix(text, "|/") || strings.HasSuffix(text, "//") {
		return text + "/"
	}

	return text + "|/"
}

func Criteria(text string, value string) string {
	if len(text) < 1 {
		return value
	}

	value = strings.ReplaceAll(value, "|", "$[vb];")
	value = strings.ReplaceAll(value, "?", "$[qu];")
	value = strings.ReplaceAll(value, "=", "$[eq];")

	return text + "?" + value
}

func AppendFetchReplace(text string, searchValue string, value string) string {
	const FS = rune(28)

	text = text[1:]

	return "@;" + searchValue + string(FS) + value + string(FS) + text
}

func LineBreak(text string, encodeLine bool) string {
	encode := ""

	if encodeLine {
		encode = "$[sln];"
	}

	text = strings.ReplaceAll(text, "\r\n", encode)
	text = strings.ReplaceAll(text, "\n", encode)
	text = strings.ReplaceAll(text, "\r", encode)

	return text
}

// Converts Numbers to Strings
func ToJSString(text string) string {
	return "\"" + text + "\""
}

// Get JS Object Momentary
func ToJSObject(text string) string {
	return "$" + text
}

// Get JS Object Returned Value Once
func ToJSReturnObject(text string) string {
	return "$@" + text
}
