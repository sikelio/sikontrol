import { Store } from 'tauri-plugin-store-api';

class StoreManager {
    private readonly store: Store;

    constructor() {
        this.store = new Store('.settings.dat');
    }

    public async setValue(key: string, value: any): Promise<void> {
        await this.store.set(key, value);
        await this.store.save();
    }

    public async getValue(key: string): Promise<unknown> {
        return await this.store.get(key);
    }

    public static getInstance() {
        return new StoreManager();
    }
}

const storeManager: StoreManager = StoreManager.getInstance();

export default storeManager;
