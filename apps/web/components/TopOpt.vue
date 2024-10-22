<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import { inject } from 'vue'
import { useToast } from '~/components/toast/use-toast'
import type { AppState } from '~/components/editor/ManualProvider.vue'
import { Job } from '@blocksuite/store'
import { getFullUrl, STATUS_FETCH } from '~/constants'
import * as http from '~/utils/http'

const { collection } = inject<AppState>('appState')!

const { toast } = useToast()

const route = useRoute()
const router = useRouter()

const noteId = route.params.id as string
const isSaving = ref(false)

const isTop = computed(() => ['/board/note', '/board/todos', '/board/diary'].includes(route.path))
const isEditor = computed(() => route.path.startsWith('/board/note/'))

const onSaveClicked = async () => {
    const noteId = route.params.id as string
    isSaving.value = true;
    const job = new Job({ collection })

    const doc = collection.getDoc(noteId)

    if (!doc) {
        toast.error({
            title: '文档不存在',
        });
        return
    }

    const tags = doc.spaceDoc.getArray<string>('tags').toArray()

    const docJson = await job.docToSnapshot(doc)

    if (!docJson) {
        toast.error({
            title: '文档转换失败',
        });
        return
    }

    docJson.meta.tags = tags

    const { status, message } = await http.patch(getFullUrl(`/api/notes`), {
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('access-token')}`,
        },
        body: JSON.stringify(docJson),
    })

    if (status === STATUS_FETCH.SUCCESS) {
        toast({
            title: '保存成功',
            description: '保存成功',
            duration: 3000,
        })
    } else {
        toast.error({
            title: '保存失败',
            description: message,
        })
    }

    isSaving.value = false;
}


</script>

<template>
    <div class="">
        <Icon v-if="isTop" icon="lucide:menu" class="text-24px cursor-pointer hover:text-[var(--accent-100)]" />
        <Icon v-else @click="router.back()" icon="lucide:arrow-left"
            class="text-24px cursor-pointer hover:text-[var(--accent-100)]" />
    </div>
    <div class="ml-auto"></div>
    <button @click="onSaveClicked" v-if="isEditor"
        class="cursor-pointer bg-[var(--accent-100)] hover:border-[var(--accent-100)] px-8px py-4px rounded-8px border-(2px solid [var(--bg-200)])">保
        存</button>
    <ThemeSwitch class="ml-16px" />
</template>