import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';

import storeManager from '../libs/StoreManager';

import type { ISocketConfig } from '../interfaces/ISocket';

export default class app_controller extends Controller {
    public static targets: string[] = ['ip', 'devices', 'port', 'token'];

    declare readonly ipTarget: HTMLSpanElement;
    declare readonly devicesTarget: HTMLUListElement;
    declare readonly portTarget: HTMLSpanElement;
    declare readonly tokenTarget: HTMLSpanElement;

    public async connect() {
        const ip: string = await invoke('get_ip');

        this.ipTarget.textContent = ip

        this.fillSocketConfig();
    }

    private async fillSocketConfig() {
        const socketConfig = await storeManager.getValue('socketConfig');

        if (socketConfig === undefined || socketConfig === null) {
            this.portTarget.textContent = 'Not set';
            this.tokenTarget.textContent = 'Not set';
        } else {
            this.portTarget.textContent = (socketConfig as ISocketConfig).port.toString();
            this.tokenTarget.textContent = (socketConfig as ISocketConfig).token;
        }
    }
}
