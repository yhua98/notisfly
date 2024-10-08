type Toast = {
	id: string;
	title: string;
	description: string;
	type: 'info' | 'success' | 'warning' | 'error';
	onClose?: () => void;
};

type State = {
	toasts: Toast[];
};

const idCreator = (() => {
	let id = 0;
	return () => {
		id++;
		return `toast-${id.toString().padStart(4, '0')}`;
	};
})();

const state = ref<State>({
	toasts: [],
});

function dispatch(toast: Toast, duration: number) {
	const clearToast = () => {
		state.value.toasts = state.value.toasts.filter((t) => t.id !== toast.id);
	};
	const close = setTimeout(() => {
		console.log('close', toast.id);
		clearToast();
	}, duration);
	const oldOnClose = toast.onClose || (() => {});
	toast.onClose = () => {
		clearTimeout(close);
		oldOnClose();
		clearToast();
	};
	state.value.toasts = [toast, ...state.value.toasts];
}

function toast(toast: Partial<Toast> & { duration: number }) {
	const id = idCreator();
	const newToast: Toast = {
		id,
		title: toast.title || '@notisfly通知',
		description: toast.description || '通知详情',
		type: toast.type || 'info',
	};
	const duration = toast.duration || 3000;

	dispatch(newToast, duration);
}

export function useToast() {
	return {
		toasts: computed(() => state.value.toasts),
		toast,
	};
}
