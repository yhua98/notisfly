export function enter(fn) {
    const that = this;
    return function (event) {
        if (event.key === 'Enter') {
            fn.call(that, event);
        }
    };
}
//# sourceMappingURL=event.js.map