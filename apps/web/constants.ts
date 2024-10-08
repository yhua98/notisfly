export const API_URL = 'http://127.0.0.1:6543';

export const getFullUrl = (path:string) => {
    return `${API_URL}${path.startsWith('/')?'':'/'}${path}`;
}