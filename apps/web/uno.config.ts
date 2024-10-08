import { defineConfig, presetAttributify, presetUno } from 'unocss';
import transformerVariantGroup from '@unocss/transformer-variant-group';

export default defineConfig({
	presets: [
		presetAttributify({
			/* preset options */
		}),
		presetUno(),
		// ...custom presets
	],
	transformers: [transformerVariantGroup()],
});
