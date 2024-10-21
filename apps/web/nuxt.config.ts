// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	modules: ['@unocss/nuxt'],
	compatibilityDate: '2024-04-03',
	devtools: { enabled: false },
	ssr: false,
	vite: {
		css: {
			preprocessorOptions: {
				scss: {
					api: 'modern-compiler'
				}
			}
		}
	}
});
