import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';

export default class app_controller extends Controller {
    public static targets: string[] = ['ip', 'devices'];

    declare readonly ipTarget: HTMLSpanElement;
    declare readonly devicesTarget: HTMLUListElement;

    public async connect() {
        const ip: string = await invoke('get_ip');

        this.ipTarget.textContent = ip
    }
}
