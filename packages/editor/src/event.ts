export function enter(this: any, fn: Function) {
    const that = this;
    return function (event: KeyboardEvent) {
        if (event.key === 'Enter') {
            fn.call(that, event);
        }
    }
}