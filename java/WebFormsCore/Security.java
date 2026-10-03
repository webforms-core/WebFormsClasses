// WebForms.java 2.2 - The Back-End Part of WebForms Core Technology, Owned by Elanat (https://elanat.net)
// Compatible with WebFormsJS version 2.2

package webformscore;

public class Security {
    public String safeValue(String value) {
        if (value == null || value.length() < 1) {
            return value;
        }
        if (value.charAt(0) == '@') {
            value = "@" + value;
        }
        value = value.replace("\n", "$[ln];")
                .replace(",@", "$[co];@")
                .replace(String.valueOf((char) 28), "\0")
                .replace(String.valueOf((char) 29), "\0")
                .replace(String.valueOf((char) 30), "\0")
                .replace(String.valueOf((char) 31), "\0");
        return value;
    }
}
