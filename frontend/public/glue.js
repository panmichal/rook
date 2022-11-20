// import { invoke } from '@tauri-apps/api/tauri';
const invoke = window.__TAURI__.invoke
export async function invokePeopleList() {
    return await invoke("people_list", {});
}