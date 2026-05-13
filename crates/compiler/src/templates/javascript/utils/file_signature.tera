export const readFileHeader = (file, length) => {
    return new Promise((resolve, reject) => {
        if (!(file instanceof Blob)) {
            resolve(null);
            return;
        }

        const reader = new FileReader();
        const blob = file.slice(0, length);

        reader.onloadend = (e) => {
            if (e.target.readyState !== FileReader.DONE) return;

            const uint = new Uint8Array(e.target.result);
            const bytes = [];

            uint.forEach((byte) => {
                bytes.push(byte.toString(16).toUpperCase().padStart(2, "0"));
            });

            resolve(bytes);
        };

        reader.onerror = () => reject(new Error("Failed to read file header."));
        reader.readAsArrayBuffer(blob);
    });
};

export const matchesSignature = (headerBytes, signatures) => {
    if (!headerBytes || !Array.isArray(signatures)) return false;

    return signatures.some((signature) => {
        if (headerBytes.length < signature.length) return false;

        for (let i = 0; i < signature.length; i++) {
            if (headerBytes[i] !== signature[i]) {
                return false;
            }
        }

        return true;
    });
};

export const getSignaturesByExtension = (ext) => {
    const signatureMap = {
        jpg: [["FF", "D8", "FF"]],
        jpeg: [["FF", "D8", "FF"]],
        png: [["89", "50", "4E", "47", "0D", "0A", "1A", "0A"]],
        gif: [
            ["47", "49", "46", "38", "37", "61"],
            ["47", "49", "46", "38", "39", "61"]
        ],
        webp: [["52", "49", "46", "46"]],
        pdf: [["25", "50", "44", "46"]],
        zip: [["50", "4B", "03", "04"]]
    };

    return signatureMap[ext] || null;
};

export const validateFileSignature = async (fileObj, fileExt) => {
    const signatures = getSignaturesByExtension(fileExt);

    if (!signatures) {
        return true;
    }

    if (!(fileObj instanceof Blob)) {
        return false;
    }

    const maxSignatureLength = Math.max(
        ...signatures.map((signature) => signature.length)
    );

    const headerBytes = await readFileHeader(fileObj, maxSignatureLength);
    return matchesSignature(headerBytes, signatures);
};