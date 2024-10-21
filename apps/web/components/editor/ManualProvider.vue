<script setup lang="ts">
import { provide } from 'vue';
import '@toeverything/theme/style.css';
import { Schema, type DocMeta } from '@blocksuite/store';
import { DocCollection } from '@blocksuite/store';
import { AffineSchemas } from '@blocksuite/blocks';

import { effects as blocksEffects } from '@blocksuite/blocks/effects'
import { effects as presetsEffects } from '@blocksuite/presets/effects'

import { ref } from 'vue';
import { getFullUrl } from '~/constants';

import * as http from '~/utils/http'

import { useToast } from '~/components/toast/use-toast'

const { toast } = useToast()

// blocksEffects();
// presetsEffects();

const schema = new Schema().register(AffineSchemas);
const collection = new DocCollection({ schema });
collection.meta.initialize();

collection.start()

const docId = ref<string>();
const metasSynced = ref(false);
const docSyncUrl = ref<(() => string) | string>();

export interface AppState {
    collection: DocCollection;
    docId: Ref<string>;
    metasSynced: Ref<boolean>;
    docSyncUrl: Ref<(() => string)>;
}

provide('appState', { collection, docId, metasSynced, docSyncUrl } as AppState);

onMounted(async () => {
    metasSynced.value = false;
    await collection.waitForSynced();
    // request short_note remote metas
    const { data, status } = await http.post<
        (DocMeta & { note_id: string, created_at: string, note_type: string })[]
    >(getFullUrl('/api/v1/notes/metas'), {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
    });
    if (status === 200 && Array.isArray(data)) {
        for (const meta of data) {
            collection.meta.addDocMeta({
                id: meta.note_id,
                title: meta.title,
                tags: meta.tags,
                createDate: new Date(meta.created_at).getTime(),
                type: meta.note_type
            });
        }
        toast({
            title: '元信息',
            description: '元信息同步成功',
            duration: 3000,
        })
    } else {
        toast({
            title: '元信息',
            description: '元信息同步失败',
            duration: 5000,
        })
    }
    metasSynced.value = true;
})
</script>

<template>
    <slot></slot>
</template>