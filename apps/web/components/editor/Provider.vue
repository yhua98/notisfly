<script setup lang="ts">
import { provide } from 'vue';
import '@toeverything/theme/style.css';
import { Doc, Schema, Job } from '@blocksuite/store';
import { DocCollection } from '@blocksuite/store';
import { AffineSchemas } from '@blocksuite/blocks';
import { WebsocketProvider } from 'y-websocket'

import { effects as blocksEffects } from '@blocksuite/blocks/effects'
import { effects as presetsEffects } from '@blocksuite/presets/effects'

blocksEffects();
presetsEffects();

import { ref } from 'vue';

const schema = new Schema().register(AffineSchemas);
const collection = new DocCollection({ schema });
collection.meta.initialize();
const provider = new WebsocketProvider('ws://127.0.0.1:6789/api/notag/ws/', 'notfly-collection', collection.doc);

const sync = new Promise((resolve) => {
    provider.once('status', (event:any) => {
        console.log(event.status);
        if (event.status === 'connected') {
            collection.start()
            resolve(true);
        }
    });
});

const docId = ref<String>();

export interface AppState {
    collection: DocCollection;
    sync: Promise<boolean>;
    docId: Ref<string>;
}

provide('appState', { collection, sync, docId } as AppState);
</script>

<template>
    <slot></slot>
</template>