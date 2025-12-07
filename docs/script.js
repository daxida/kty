const languages = [];

// The hugging face repo id
const repoId = "daxida/test-dataset"
// Care about the "blob" instead of "tree"
const latestUrl = `https://huggingface.co/datasets/${repoId}/resolve/main/dict`;

// TODO: these versions should be stored somewhere in the repo instead of fetched...

// The dictionary version (calver), different from the crate version
async function getDictionariesVersion() {
    const url = `https://huggingface.co/datasets/${repoId}/raw/main/README.md`;
    const text = await fetch(url).then(r => r.text());
    const match = text.match(/version:\s*([^\n]+)\n/);
    const version = match?.[1] || "no-version-found";
    return `dic v${version}`
}

// Get the crate version from https://github.com/daxida/kty/blob/master/Cargo.toml
async function getCrateVersion() {
    const url = "https://raw.githubusercontent.com/daxida/kty/master/Cargo.toml";
    const text = await fetch(url).then(r => r.text());
    const match = text.match(/^version\s*=\s*"([^"]+)"/m);
    const version = match?.[1] || "no-version-found";
    return `kty v${version}`
}

async function populateVersion() {
    const dictionaryVersion = await getDictionariesVersion();
    const crateVersion = await getCrateVersion();
    const dictElem = document.getElementById("version-dictionary");
    const crateElem = document.getElementById("version-crate");
    if (dictElem) dictElem.textContent = dictionaryVersion;
    if (crateElem) crateElem.textContent = crateVersion;
}

async function fetchLanguages() {
    try {
        const resp = await fetch(
            "https://raw.githubusercontent.com/daxida/kty/refs/heads/master/assets/languages.json",
        );
        if (!resp.ok) throw new Error("Failed to fetch languages");
        const data = await resp.json();
        languages.push(...data);
        return data;
    } catch (err) {
        console.error("Error fetching languages:", err);
        throw err;
    }
}

const allLangs = [];
const editionLangs = [];
// iso > lang -- not sure why it is needed over allLangs
const langMap = {};

function updateLanguageData() {
    allLangs.length = 0;
    editionLangs.length = 0;
    for (const k in langMap) delete langMap[k];

    allLangs.push(...languages.filter((l) => l.language));
    editionLangs.push(...languages.filter((l) => l.hasEdition));
    for (const l of allLangs) langMap[l.iso] = l;
}

function dropdownOptionNode({ iso, language, displayName, flag }) {
    const opt = document.createElement("option");
    opt.value = iso;
    opt.textContent = `${flag} ${displayName || language}`;
    return opt;
}

function populateDropdown(selector, items, includeMerged = false) {
    const el = document.querySelector(selector);
    if (!el) return;
    el.innerHTML = "";
    // items are already sorted per displayName
    const list = includeMerged
        ? [
            {
                iso: "merged",
                language: "Merged",
                displayName: "Merged",
                flag: "🧬",
            },
            ...items,
        ]
        : items;
    for (const item of list) el.appendChild(dropdownOptionNode(item));
}

function updateDownloadLink(tgtSel, glossSel, linkSel, type) {
    const tgtEl = document.querySelector(tgtSel);
    const glossEl = document.querySelector(glossSel);
    const linkEl = document.querySelector(linkSel);
    if (!tgtEl || !glossEl || !linkEl) return;

    const target = tgtEl.value;
    const source = glossEl.value;

    let url = "";
    switch (type) {
        case "main":
            url = `${latestUrl}/${target}/${source}/kty-${target}-${source}.zip`;
            break;
        case "ipa":
            url =
                source === "merged"
                    ? `${latestUrl}kty-${target}-ipa.zip`
                    : `${latestUrl}kty-${target}-${source}-ipa.zip`;
            break;
        case "translations":
            url = `${latestUrl}kty-${target}-${source}-gloss.zip`;
            break;
        default:
            console.error("Unknown type:", type);
    }

    url = `${url}?download=true`;

    linkEl.setAttribute("href", url);
}

// A prefix can be "main", "ipa". Same for type.
function setupDropdowns(prefix, type, isIPA = false) {
    populateDropdown(`#${prefix}-target`, allLangs);
    populateDropdown(`#${prefix}-gloss`, editionLangs, isIPA);

    function handler() {
        updateDownloadLink(
            `#${prefix}-target`,
            `#${prefix}-gloss`,
            `#${prefix}-download`,
            type
        );
    }

    handler()

    const target = document.querySelector(`#${prefix}-target`);
    const gloss = document.querySelector(`#${prefix}-gloss`);

    [target, gloss].forEach(el => {
        if (el) el.addEventListener("change", handler);
    });
}

function validateTranslationsDropdowns() {
    /** @type {HTMLSelectElement | null} */
    const targetEl = document.querySelector("#trans-target");
    /** @type {HTMLSelectElement | null} */
    const glossEl = document.querySelector("#trans-gloss");

    if (!targetEl || !glossEl) return;

    const targetValue = targetEl.value;
    const glossValue = glossEl.value;

    if (targetValue === glossValue) {
        const availableGloss = allLangs.find(
            (lang) => lang.iso !== targetValue,
        );
        if (availableGloss) {
            glossEl.value = availableGloss.iso;
        }
    }

    updateDownloadLink(
        "#trans-target",
        "#trans-gloss",
        "#trans-download",
        "translations",
    );
}

document.addEventListener("DOMContentLoaded", async () => {
    try {
        await populateVersion();

        await fetchLanguages();
        updateLanguageData();

        setupDropdowns("main", "main");
        setupDropdowns("ipa", "ipa", (isIPA = true));

        populateDropdown("#trans-target", editionLangs);
        populateDropdown("#trans-gloss", allLangs);

        validateTranslationsDropdowns();

        const transTarget = document.querySelector("#trans-target");
        const transGloss = document.querySelector("#trans-gloss");

        if (transTarget) {
            transTarget.addEventListener("change", () => {
                validateTranslationsDropdowns();
            });
        }

        if (transGloss) {
            transGloss.addEventListener("change", () => {
                validateTranslationsDropdowns();
            });
        }
    } catch (err) {
        console.error("Error initializing page:", err);
    }
});
