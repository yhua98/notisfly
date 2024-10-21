import type { NoteBlock, NoteMeta } from "./types";

export const API_URL = 'http://127.0.0.1:6543';

export const STATUS_FETCH = {
    SUCCESS: 2000,
    ERROR: 4000
}

export const getFullUrl = (path: string) => {
    return `${API_URL}${path.startsWith('/') ? '' : '/'}${path}`;
}

export const createDocJson = (id: string, meta: NoteMeta, blocks: NoteBlock[]) => ({
    type: "page",
    meta: meta,
    // {
    //     id: "1ii3awWOm1",
    //     title: "",
    //     createDate: 1729413924695,
    //     tags: []
    // },
    blocks: {
        type: "block",
        id: id,
        flavour: "affine:page",
        version: 2,
        props: {
            title: {
                "$blocksuite:internal:text$": true,
                delta: [
                    { insert: meta.title }
                ]
            }
        },
        children: [
            // {
            //     type: "block",
            //     id: "7BGtlmSVQY",
            //     flavour:
            //         "affine:surface",
            //     version: 5,
            //     props: { elements: {} },
            //     children: []
            // },
            {
                type: "block",
                id: "W-lPW5mA7G",
                flavour: "affine:note",
                version: 1,
                props: {
                    xywh: "[0,0,800,95]",
                    background: "--affine-note-background-white",
                    index: "a0",
                    hidden: false,
                    displayMode: "both",
                    edgeless: {
                        style: {
                            borderRadius: 8,
                            borderSize: 4,
                            borderStyle: "none",
                            shadowType: "--affine-note-shadow-box"
                        }
                    }
                },
                children: blocks
            }
        ]
    }
})