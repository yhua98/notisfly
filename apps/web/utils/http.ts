type ResponsePayload<T> = {
    status: number;
    message: string;
    data?: T;
}


export async function post<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, init)
    const { status, data, message } = await res.json()

    return { status, data: data, message }
}

export async function get<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, init)
    const { status, data, message } = await res.json()

    return { status, data, message }
}

export async function put<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, init)
    const { status, data, message } = await res.json()

    return { status, data, message }
}

export async function patch<T>(input: string | URL | globalThis.Request, init?: RequestInit): Promise<ResponsePayload<T>> {
    const res = await fetch(input, init)
    const { status, data, message } = await res.json()

    return { status, data, message }
}