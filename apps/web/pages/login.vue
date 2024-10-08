<script lang="ts" setup>

import { ref } from "vue"
import { getFullUrl } from '~/constants'

const router = useRouter()

definePageMeta({
    layout: 'data-board'
})

const isLogin = ref(true)

const username = ref("")
const useremail = ref("")
const password = ref("")

const onSubmit = async () => {

    if (!isLogin.value && username.value === "") {
        console.log("用户昵称不能为空")
        return
    }
    if (useremail.value === "") {
        console.log("用户邮箱不能为空")
        return
    }
    if (password.value === "") {
        console.log("用户密码不能为空")
        return
    }
    if (isLogin.value) {
        // login
        const result = await fetch(getFullUrl('/api/user/auth/login'), {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                email: useremail.value,
                password: password.value
            })
        })
        const { status, message, data } = await result.json()

        console.log(status, message, data, router)

        if (status === 200) {
            localStorage.setItem('access-token', data.token)
            router.replace({
                path: '/databoard'
            })
        }
    } else {
        // registry
        const result = await fetch(getFullUrl('/api/user/registry'), {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: username.value,
                email: useremail.value,
                password: password.value
            })
        })
        const { status, message, data } = await result.json()

        console.log(status, message, data, router)


    }
}

const changeOpt = () => {
    isLogin.value = !isLogin.value
}

</script>

<template>
    <div class="h-100vh flex justify-center items-center">
        <div class="overflow-hidden relative shadow flex flex-col items-center 
        p-16px w-500px min-h-400px bg-[var(--bg-200)] rounded-16px">
            <div class="mt-32px text-3rem bg-gradient-to-r bg-clip-text bg-gradient-from-blue bg-gradient-to-green">
                <div class="text-transparent ">@Notisfly</div>
            </div>
            <div v-if="!isLogin" data-name="昵称" class="group mt-32px relative 
            before:(content-[attr(data-name)] transition-all pointer-events-none absolute z-2 top-10px left-18px text-green ) 
            after:(content-[attr(data-name)] pointer-events-none w-[2.5em] -translate-x-0.25em absolute z-1 
            top-10px left-18px text-transparent bg-[var(--bg-200)] ) 
            has-[input:focus]:(before:(-top-0.75em) after:(-top-0.75em))
            has-[input[data-have='true']]:(before:(-top-0.75em) after:(-top-0.75em))
            ">
                <input v-model="username" :data-have="!!username" data-name="昵称" type="text" class="
                text-[var(--text-100)]
                transition-all
                min-w-300px rounded-8px py-8px px-16px 
            outline-none focus:border-[var(--accent-100)] 
            border-(2px solid [var(--bg-300)]) bg-transparent" />
            </div>
            <div data-name="邮箱" class="group mt-16px first:mt-32px relative 
            before:(content-[attr(data-name)] transition-all pointer-events-none absolute z-2 top-10px left-18px text-green ) 
            after:(content-[attr(data-name)] pointer-events-none w-[2.5em] -translate-x-0.25em absolute z-1 
            top-10px left-18px text-transparent bg-[var(--bg-200)] ) 
            has-[input:focus]:(before:(-top-0.75em) after:(-top-0.75em))
            has-[input[data-have='true']]:(before:(-top-0.75em) after:(-top-0.75em))
            ">
                <input type="email" required v-model="useremail" :data-have="!!useremail" data-name="邮箱" class="
                text-[var(--text-100)]
                transition-all
                min-w-300px rounded-8px py-8px px-16px 
            outline-none focus:border-[var(--accent-100)] 
            border-(2px solid [var(--bg-300)]) bg-transparent" />
            </div>
            <div data-name="密码" class="group mt-16px relative 
            before:(content-[attr(data-name)] transition-all pointer-events-none absolute z-2 top-10px left-18px text-green ) 
            after:(content-[attr(data-name)] pointer-events-none w-[2.5em] -translate-x-0.25em absolute z-1 
            top-10px left-18px text-transparent bg-[var(--bg-200)] ) 
            has-[:focus]:(before:(-top-0.75em) after:(-top-0.75em))
            has-[input[data-have='true']]:(before:(-top-0.75em) after:(-top-0.75em))
            ">
                <input type="password" required v-model="password" :data-have="!!password" data-name="邮箱" class="
                text-[var(--text-100)]
                transition-all
                min-w-300px rounded-8px py-8px px-16px 
            outline-none focus:border-[var(--accent-100)] 
            border-(2px solid [var(--bg-300)]) bg-transparent" />
            </div>

            <div class="flex w-300px mt-8px">
                <div class="ml-auto mr-16px cursor-pointer hover:text-[var(--text-100)] text-blue">忘记密码</div>
            </div>

            <div class="mt-32px">
                <button @click="onSubmit"
                    class="rounded-8px bg-[var(--accent-100)] hover:border-[var(--bg-300)] py-8px px-16px border-(2px solid [var(--accent-100)])">
                    {{ isLogin ? '登 录' : '注 册' }}
                </button>
            </div>

            <div @click="changeOpt"
                class="cursor-pointer flex justify-center rotate-[45deg] w-200px translate-x-85px p-8px absolute top-0 right-0 bg-[var(--bg-300)]">
                <div class="select-none text-1em text-[var(--text-200)] leading-[1.25em]">{{ isLogin ? '注册' : '登 录' }}
                </div>
            </div>
        </div>
    </div>
</template>