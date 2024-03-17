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
        [key: string]: string | ITomlDependencyCrates;
    };
    features: {
        'custom-protocol': string[]
    }
}

export interface ITomlDependencyCrates {
    version: string,
    features: string[]
}

export interface ITomlDependencyGitHub {
    git: string;
    branch: string;
}
