import { createHighlighter, type Highlighter } from 'shiki';

let instance = $state<Highlighter | null>(null);
let loadingPromise = $state<Promise<Highlighter> | null>(null);

export function getHighlighter(languages: string[]) {
    if (instance) return Promise.resolve(instance);

    if (loadingPromise) return loadingPromise;

    loadingPromise = createHighlighter({
        themes: ['github-dark'],
        langs: languages
    }).then(h => {
        instance = h;
        return h;
    });

    return loadingPromise;
}