import type { ResponsePayload } from '../types'

export async function post<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, { ...init, method: 'POST' })
    const { status, data, message } = await res.json()

    return { status, data: data, message }
}

export async function get<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, { ...init, method: 'GET' })
    const { status, data, message } = await res.json()

    return { status, data, message }
}

export async function put<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, init)
    const { status, data, message } = await res.json()

    return { status, data, message }
}

export async function patch<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, { ...init, method: 'PATCH' })
    const { status, data, message } = await res.json()

    return { status, data, message }
}