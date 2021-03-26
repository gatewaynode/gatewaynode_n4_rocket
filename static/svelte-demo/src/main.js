import App from './App.svelte';

const demo = new App({
	// target: document.body,
	props: {
		something: 'Default'
	}
});

export default demo;