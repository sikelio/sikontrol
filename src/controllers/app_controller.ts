import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';

import CustomAlert from '../libs/CustomAlert';
import storeManager from '../libs/StoreManager';

import type { ISocketConfig } from '../interfaces/ISocket';

export default class app_controller extends Controller {
    public static targets: string[] = ['ip', 'devices', 'port', 'token', 'startbtn', 'stopbtn', 'servivestatus'];

    declare readonly ipTarget: HTMLSpanElement;
    declare readonly devicesTarget: HTMLUListElement;
    declare readonly portTarget: HTMLSpanElement;
    declare readonly tokenTarget: HTMLSpanElement;
    declare readonly startbtnTarget: HTMLButtonElement;
    declare readonly stopbtnTarget: HTMLButtonElement;
    declare readonly servivestatusTarget: HTMLSpanElement;

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

    public async startServer(e: MouseEvent) {
        e.preventDefault();

        try {
            const socketConfig = await storeManager.getValue('socketConfig');

            if (socketConfig === undefined || socketConfig === null) {
                return CustomAlert.Toast.fire({
                    icon: 'warning',
                    title: 'Settings error',
                    text: 'You must set some settings, at least a port.'
                });
            }            

            this.startbtnTarget.disabled = true;
            this.stopbtnTarget.disabled = false;
            this.servivestatusTarget.innerText = 'Started';

            await invoke('start_server', { port: (socketConfig as ISocketConfig).port });
        } catch (err: any) {
            this.startbtnTarget.disabled = false;
            this.stopbtnTarget.disabled = true;
            this.servivestatusTarget.innerText = 'Stopped';
        }
    }

    public stopServer(e: MouseEvent) {
        e.preventDefault();

        try {
            this.startbtnTarget.disabled = false;
            this.stopbtnTarget.disabled = true;
            this.servivestatusTarget.innerText = 'Stopped';

            // await invoke('stop_server');
        } catch (err: any) {
            this.startbtnTarget.disabled = true;
            this.stopbtnTarget.disabled = false;
            this.servivestatusTarget.innerText = 'Started';
        }
    }
}
