export interface IToml {
    package: {
        name: string;
        version: string;
        description: string;
        authors: string[];
        edition: string
    };
    'build-dependencies': {
        'tauri-build': {
            version: string;
            features: any[]
        }
    };
    dependencies: {
        [key: string]: string | ITomlDependency;
    };
    features: {
        'custom-protocol': string[]
    }
}

export interface ITomlDependency {
    version: string,
    features: string[]
}
