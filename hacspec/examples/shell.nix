{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	buildInputs = [
		pkgs.libclang
	];
	shellHook = ''
		export LIBCLANG_PATH="${pkgs.libclang}/lib"
	'';
}
