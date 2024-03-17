export interface IPackageJson {
    name: string;
    private: boolean;
    version: string;
    type: 'module' | 'commonjs';
    author: string;
    homepage: string;
    bugs: {
        url: string;
    };
    scripts: { [key: string]: string };
    dependencies: { [key: string]: string };
    devDependencies: { [key: string]: string };
}
