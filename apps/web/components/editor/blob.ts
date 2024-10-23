import { getFullUrl } from "~/constants";
import * as http from "~/utils/http";

export interface BlobSource {
    name: string;
    readonly: boolean;
    get: (key: string) => Promise<Blob | null>;
    set: (key: string, value: Blob) => Promise<string>;
    delete: (key: string) => Promise<void>;
    list: () => Promise<string[]>;
}

let cache = new Map<string, boolean>();

function blobToArrayBuffer(blob: Blob) {
    return new Promise<ArrayBuffer>((resolve, reject) => {
        let reader = new FileReader();
        reader.onload = () => resolve(reader.result as ArrayBuffer);
        reader.onerror = reject;
        reader.readAsArrayBuffer(blob);
    });
}

export class BlobStorage implements BlobSource {
    name: string;
    readonly: boolean;

    constructor(name: string, readonly: boolean) {
        this.name = name;
        this.readonly = readonly;
    }

    async get(key: string) {
        // use cache
        // get from server
        const url = getFullUrl(`/api/oss/${key}`);
        const { data } = await http.get(url, {
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${localStorage.getItem('access-token')}`
            }
        })
        if (!data) {
            return null;
        }
        console.log('data', data)

        const blob = new Blob([new Uint8Array(data.data)], { type: 'image/png' });

        return Promise.resolve(blob);
    }
    async set(key: string, value: Blob) {

        console.log('set', key, value)

        const data = new FormData();

        data.append('file', value);
        data.append('key', key);

        // upload to server
        await http.post(getFullUrl(`/api/oss`), {
            body: data,
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
            }
        })
        return key;
    }
    delete(key: string) {
        return Promise.resolve();
    }
    list() {
        return Promise.resolve([]);
    }

}