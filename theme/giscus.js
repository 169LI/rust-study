(() => {
    const GISCUS_ORIGIN = "https://giscus.app";
    const DARK_THEMES = new Set(["coal", "navy", "ayu"]);

    function getMdBookTheme() {
        return document.documentElement.className
            .split(/\s+/)
            .find((name) => ["light", "rust", "coal", "navy", "ayu"].includes(name));
    }

    function getGiscusTheme() {
        const theme = getMdBookTheme();

        if (DARK_THEMES.has(theme)) {
            return "dark";
        }

        if (theme === "light" || theme === "rust") {
            return "light";
        }

        return window.matchMedia("(prefers-color-scheme: dark)").matches
            ? "dark"
            : "light";
    }

    function updateGiscusTheme() {
        const iframe = document.querySelector("iframe.giscus-frame");

        if (!iframe) {
            return;
        }

        iframe.contentWindow.postMessage(
            {
                giscus: {
                    setConfig: {
                        theme: getGiscusTheme(),
                    },
                },
            },
            GISCUS_ORIGIN,
        );
    }

    function createGiscus() {
        const main = document.querySelector("#mdbook-content main");

        if (!main || main.querySelector(".giscus-comments")) {
            return;
        }

        const comments = document.createElement("section");
        comments.className = "giscus-comments";
        comments.setAttribute("aria-label", "讨论区");

        const heading = document.createElement("h2");
        heading.textContent = "讨论区";
        comments.appendChild(heading);

        const script = document.createElement("script");
        const attributes = {
            src: `${GISCUS_ORIGIN}/client.js`,
            "data-repo": "169LI/rust-study",
            "data-repo-id": "R_kgDOQwBz4g",
            "data-category": "Comments",
            "data-category-id": "DIC_kwDOQwBz4s4DBUtV",
            "data-mapping": "pathname",
            "data-strict": "0",
            "data-reactions-enabled": "1",
            "data-emit-metadata": "0",
            "data-input-position": "top",
            "data-theme": getGiscusTheme(),
            "data-lang": "zh-CN",
            //"data-loading": "lazy",
            crossorigin: "anonymous",
        };

        Object.entries(attributes).forEach(([name, value]) => {
            script.setAttribute(name, value);
        });
        script.async = true;

        comments.appendChild(script);
        main.appendChild(comments);
    }

    document.addEventListener("DOMContentLoaded", () => {
        createGiscus();

        new MutationObserver(updateGiscusTheme).observe(document.documentElement, {
            attributes: true,
            attributeFilter: ["class"],
        });

        const colorScheme = window.matchMedia("(prefers-color-scheme: dark)");
        colorScheme.addEventListener?.("change", updateGiscusTheme);
    });
})();
