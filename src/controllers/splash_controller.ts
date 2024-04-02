import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';

export default class splash_controller extends Controller {
    public connect(): void {
        setTimeout(() => {
            invoke('close_splashscreen');
        }, 5000);
    }
}
