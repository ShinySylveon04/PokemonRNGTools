parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"JZPE":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.createEndpoint=void 0,exports.expose=c,exports.proxy=h,exports.releaseProxy=exports.proxyMarker=void 0,exports.transfer=y,exports.transferHandlers=void 0,exports.windowEndpoint=v,exports.wrap=l;const e=Symbol("Comlink.proxy");exports.proxyMarker=e;const t=Symbol("Comlink.endpoint");exports.createEndpoint=t;const n=Symbol("Comlink.releaseProxy");exports.releaseProxy=n;const r=Symbol("Comlink.thrown"),a=e=>"object"==typeof e&&null!==e||"function"==typeof e,s={canHandle:t=>a(t)&&t[e],serialize(e){const{port1:t,port2:n}=new MessageChannel;return c(e,t),[n,[n]]},deserialize:e=>(e.start(),l(e))},o={canHandle:e=>a(e)&&r in e,serialize({value:e}){let t;return[t=e instanceof Error?{isError:!0,value:{message:e.message,name:e.name,stack:e.stack}}:{isError:!1,value:e},[]]},deserialize(e){if(e.isError)throw Object.assign(new Error(e.value.message),e.value);throw e.value}},i=new Map([["proxy",s],["throw",o]]);function c(e,t=self){t.addEventListener("message",function n(a){if(!a||!a.data)return;const{id:s,type:o,path:i}=Object.assign({path:[]},a.data),u=(a.data.argumentList||[]).map(x);let l;try{const t=i.slice(0,-1).reduce((e,t)=>e[t],e),n=i.reduce((e,t)=>e[t],e);switch(o){case"GET":l=n;break;case"SET":t[i.slice(-1)[0]]=x(a.data.value),l=!0;break;case"APPLY":l=n.apply(t,u);break;case"CONSTRUCT":l=h(new n(...u));break;case"ENDPOINT":{const{port1:t,port2:n}=new MessageChannel;c(e,n),l=y(t,[t])}break;case"RELEASE":l=void 0;break;default:return}}catch(d){l={value:d,[r]:0}}Promise.resolve(l).catch(e=>({value:e,[r]:0})).then(e=>{const[r,a]=b(e);t.postMessage(Object.assign(Object.assign({},r),{id:s}),a),"RELEASE"===o&&(t.removeEventListener("message",n),p(t))})}),t.start&&t.start()}function u(e){return"MessagePort"===e.constructor.name}function p(e){u(e)&&e.close()}function l(e,t){return f(e,[],t)}function d(e){if(e)throw new Error("Proxy has been released and is not useable")}function f(e,r=[],a=function(){}){let s=!1;const o=new Proxy(a,{get(t,a){if(d(s),a===n)return()=>w(e,{type:"RELEASE",path:r.map(e=>e.toString())}).then(()=>{p(e),s=!0});if("then"===a){if(0===r.length)return{then:()=>o};const t=w(e,{type:"GET",path:r.map(e=>e.toString())}).then(x);return t.then.bind(t)}return f(e,[...r,a])},set(t,n,a){d(s);const[o,i]=b(a);return w(e,{type:"SET",path:[...r,n].map(e=>e.toString()),value:o},i).then(x)},apply(n,a,o){d(s);const i=r[r.length-1];if(i===t)return w(e,{type:"ENDPOINT"}).then(x);if("bind"===i)return f(e,r.slice(0,-1));const[c,u]=E(o);return w(e,{type:"APPLY",path:r.map(e=>e.toString()),argumentList:c},u).then(x)},construct(t,n){d(s);const[a,o]=E(n);return w(e,{type:"CONSTRUCT",path:r.map(e=>e.toString()),argumentList:a},o).then(x)}});return o}function m(e){return Array.prototype.concat.apply([],e)}function E(e){const t=e.map(b);return[t.map(e=>e[0]),m(t.map(e=>e[1]))]}exports.transferHandlers=i;const g=new WeakMap;function y(e,t){return g.set(e,t),e}function h(t){return Object.assign(t,{[e]:!0})}function v(e,t=self,n="*"){return{postMessage:(t,r)=>e.postMessage(t,n,r),addEventListener:t.addEventListener.bind(t),removeEventListener:t.removeEventListener.bind(t)}}function b(e){for(const[t,n]of i)if(n.canHandle(e)){const[r,a]=n.serialize(e);return[{type:"HANDLER",name:t,value:r},a]}return[{type:"RAW",value:e},g.get(e)||[]]}function x(e){switch(e.type){case"HANDLER":return i.get(e.name).deserialize(e.value);case"RAW":return e.value}}function w(e,t,n){return new Promise(r=>{const a=L();e.addEventListener("message",function t(n){n.data&&n.data.id&&n.data.id===a&&(e.removeEventListener("message",t),r(n.data))}),e.start&&e.start(),e.postMessage(Object.assign({id:a},t),n)})}function L(){return new Array(4).fill(0).map(()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16)).join("-")}
},{}],"x0cg":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.init_panic_hook=exports.get_wild=exports.get_bdsp_tid=exports.default=exports.calculate_pokemon_bdsp_underground=exports.calculate_pokemon_bdsp_stationary=exports.calculate_pokemon_bdsp_roamer=exports.calculate_pokemon=exports.__wbindgen_throw=exports.__wbindgen_string_new=exports.__wbindgen_string_get=exports.__wbindgen_object_drop_ref=exports.__wbindgen_json_serialize=exports.__wbindgen_json_parse=exports.__wbg_stack_0ddaca5d1abfb52f=exports.__wbg_shinyresult_new=exports.__wbg_push_284486ca27c6aa8b=exports.__wbg_new_949bbc1147195c4e=exports.__wbg_new_693216e109162396=exports.__wbg_error_09919627ac0992f5=exports.Xorshift=exports.Xoroshiro=exports.ShinyResultBdspStationary=exports.ShinyResult=exports.ShinyFilter=exports.Shiny=exports.Result=exports.Pokemon=exports.NatureFilter=exports.Nature=exports.LeadFilter=exports.GenderRatio=exports.GenderFilter=exports.Gender=exports.EncounterSlotFilter=exports.EncounterSlot=exports.EncounterFilter=exports.AbilityFilter=exports.Ability=void 0;var e=t(require("./pkg/wasm_bg.wasm"));function t(e){return e&&e.__esModule?e:{default:e}}var r=e.default;exports.default=r;var _=e.default.init_panic_hook;exports.init_panic_hook=_;var o=e.default.get_wild;exports.get_wild=o;var a=e.default.get_bdsp_tid;exports.get_bdsp_tid=a;var s=e.default.calculate_pokemon;exports.calculate_pokemon=s;var n=e.default.calculate_pokemon_bdsp_stationary;exports.calculate_pokemon_bdsp_stationary=n;var l=e.default.calculate_pokemon_bdsp_underground;exports.calculate_pokemon_bdsp_underground=l;var p=e.default.calculate_pokemon_bdsp_roamer;exports.calculate_pokemon_bdsp_roamer=p;var d=e.default.__wbindgen_json_serialize;exports.__wbindgen_json_serialize=d;var i=e.default.__wbindgen_json_parse;exports.__wbindgen_json_parse=i;var u=e.default.__wbg_new_949bbc1147195c4e;exports.__wbg_new_949bbc1147195c4e=u;var x=e.default.__wbg_shinyresult_new;exports.__wbg_shinyresult_new=x;var b=e.default.__wbg_push_284486ca27c6aa8b;exports.__wbg_push_284486ca27c6aa8b=b;var c=e.default.__wbindgen_object_drop_ref;exports.__wbindgen_object_drop_ref=c;var f=e.default.__wbg_new_693216e109162396;exports.__wbg_new_693216e109162396=f;var g=e.default.__wbg_stack_0ddaca5d1abfb52f;exports.__wbg_stack_0ddaca5d1abfb52f=g;var w=e.default.__wbg_error_09919627ac0992f5;exports.__wbg_error_09919627ac0992f5=w;var v=e.default.__wbindgen_string_new;exports.__wbindgen_string_new=v;var h=e.default.__wbindgen_string_get;exports.__wbindgen_string_get=h;var y=e.default.__wbindgen_throw;exports.__wbindgen_throw=y;var k=e.default.LeadFilter;exports.LeadFilter=k;var F=e.default.AbilityFilter;exports.AbilityFilter=F;var S=e.default.Ability;exports.Ability=S;var m=e.default.NatureFilter;exports.NatureFilter=m;var R=e.default.Nature;exports.Nature=R;var j=e.default.ShinyFilter;exports.ShinyFilter=j;var E=e.default.EncounterFilter;exports.EncounterFilter=E;var G=e.default.Shiny;exports.Shiny=G;var A=e.default.EncounterSlotFilter;exports.EncounterSlotFilter=A;var N=e.default.EncounterSlot;exports.EncounterSlot=N;var X=e.default.GenderRatio;exports.GenderRatio=X;var P=e.default.Gender;exports.Gender=P;var z=e.default.GenderFilter;exports.GenderFilter=z;var B=e.default.Pokemon;exports.Pokemon=B;var L=e.default.Result;exports.Result=L;var M=e.default.ShinyResult;exports.ShinyResult=M;var q=e.default.ShinyResultBdspStationary;exports.ShinyResultBdspStationary=q;var O=e.default.Xoroshiro;exports.Xoroshiro=O;var C=e.default.Xorshift;exports.Xorshift=C;
},{"./pkg/wasm_bg.wasm":"lGJG"}],"njwa":[function(require,module,exports) {
"use strict";var e=require("comlink"),r=require("../../../../wasm/Cargo.toml");(0,e.expose)(r.calculate_pokemon_bdsp_stationary);
},{"comlink":"JZPE","../../../../wasm/Cargo.toml":"x0cg"}],"FheM":[function(require,module,exports) {
var t=null;function e(){return t||(t=n()),t}function n(){try{throw new Error}catch(e){var t=(""+e.stack).match(/(https?|file|ftp|chrome-extension|moz-extension):\/\/[^)\n]+/g);if(t)return r(t[0])}return"/"}function r(t){return(""+t).replace(/^((?:https?|file|ftp|chrome-extension|moz-extension):\/\/.+)?\/[^/]+(?:\?.*)?$/,"$1")+"/"}exports.getBundleURL=e,exports.getBaseURL=r;
},{}],"TUK3":[function(require,module,exports) {
var r=require("./bundle-url").getBundleURL;function e(r){Array.isArray(r)||(r=[r]);var e=r[r.length-1];try{return Promise.resolve(require(e))}catch(n){if("MODULE_NOT_FOUND"===n.code)return new s(function(n,i){t(r.slice(0,-1)).then(function(){return require(e)}).then(n,i)});throw n}}function t(r){return Promise.all(r.map(u))}var n={};function i(r,e){n[r]=e}module.exports=exports=e,exports.load=t,exports.register=i;var o={};function u(e){var t;if(Array.isArray(e)&&(t=e[1],e=e[0]),o[e])return o[e];var i=(e.substring(e.lastIndexOf(".")+1,e.length)||e).toLowerCase(),u=n[i];return u?o[e]=u(r()+e).then(function(r){return r&&module.bundle.register(t,r),r}).catch(function(r){throw delete o[e],r}):void 0}function s(r){this.executor=r,this.promise=null}s.prototype.then=function(r,e){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.then(r,e)},s.prototype.catch=function(r){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.catch(r)};
},{"./bundle-url":"FheM"}],"rDCW":[function(require,module,exports) {

},{}],"fISM":[function(require,module,exports) {
var __dirname = "/home/runner/work/PokemonRNGTools/PokemonRNGTools/node_modules/parcel-plugin-wasm.rs";
var t,e="/home/runner/work/PokemonRNGTools/PokemonRNGTools/node_modules/parcel-plugin-wasm.rs";const _={},r=new Array(32).fill(void 0);function s(t){return r[t]}r.push(void 0,null,!0,!1);let n=0,i=null;function a(){return null!==i&&i.buffer===t.memory.buffer||(i=new Uint8Array(t.memory.buffer)),i}const o="undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder;let l=new o("utf-8");const u="function"==typeof l.encodeInto?function(t,e){return l.encodeInto(t,e)}:function(t,e){const _=l.encode(t);return e.set(_),{read:t.length,written:_.length}};function g(t,e,_){if(void 0===_){const _=l.encode(t),r=e(_.length);return a().subarray(r,r+_.length).set(_),n=_.length,r}let r=t.length,s=e(r);const i=a();let o=0;for(;o<r;o++){const e=t.charCodeAt(o);if(e>127)break;i[s+o]=e}if(o!==r){0!==o&&(t=t.slice(o)),s=_(s,r,r=o+3*t.length);const e=a().subarray(s+o,s+r);o+=u(t,e).written}return n=o,s}let b=null;function d(){return null!==b&&b.buffer===t.memory.buffer||(b=new Int32Array(t.memory.buffer)),b}const c="undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder;let h=new c("utf-8",{ignoreBOM:!0,fatal:!0});function y(t,e){return h.decode(a().subarray(t,t+e))}h.decode();let p=r.length;function w(t){p===r.length&&r.push(r.length+1);const e=p;return p=r[e],r[e]=t,e}function f(t){t<36||(r[t]=p,p=t)}function m(t){const e=s(t);return f(t),e}function S(t){return null==t}_.init_panic_hook=function(){t.init_panic_hook()};let v=32;function k(t){if(1==v)throw new Error("out of js stack");return r[--v]=t,v}_.get_wild=function(e){try{return m(t.get_wild(k(e)))}finally{r[v++]=void 0}},_.get_bdsp_tid=function(e){try{return m(t.get_bdsp_tid(k(e)))}finally{r[v++]=void 0}};const A=new Uint32Array(2),M=new BigUint64Array(A.buffer);let F=null;function N(){return null!==F&&F.buffer===t.memory.buffer||(F=new Uint32Array(t.memory.buffer)),F}function j(t,e){return N().subarray(t/4,t/4+e)}function B(t,e){const _=e(4*t.length);return N().set(t,_/4),n=t.length,_}_.calculate_pokemon=function(e,_,r,s,n,i,a,o,l,u,g){M[0]=e;const b=A[0],d=A[1];M[0]=_;const c=A[0],h=A[1];return m(t.calculate_pokemon(b,d,c,h,r,s,n,i,a,o,l,u,g))},_.calculate_pokemon_bdsp_stationary=function(e,_,r,s,i,a,o,l,u,g,b,d,c,h,y,p){var w=B(u,t.__wbindgen_malloc),f=n,S=B(h,t.__wbindgen_malloc),v=n,k=B(y,t.__wbindgen_malloc),A=n;return m(t.calculate_pokemon_bdsp_stationary(e,_,r,s,i,a,o,l,w,f,g,b,d,c,S,v,k,A,p))},_.calculate_pokemon_bdsp_underground=function(e,_,r,s,i,a,o,l,u,g,b,d,c,h,y,p,w,f){var S=B(u,t.__wbindgen_malloc),v=n,k=B(w,t.__wbindgen_malloc),A=n,M=B(f,t.__wbindgen_malloc),F=n;return m(t.calculate_pokemon_bdsp_underground(e,_,r,s,i,a,o,l,S,v,g,b,d,c,h,y,p,k,A,M,F))},_.calculate_pokemon_bdsp_roamer=function(e,_,r,s,i,a,o,l,u,g,b,d,c,h,y){var p=B(u,t.__wbindgen_malloc),w=n,f=B(h,t.__wbindgen_malloc),S=n,v=B(y,t.__wbindgen_malloc),k=n;return m(t.calculate_pokemon_bdsp_roamer(e,_,r,s,i,a,o,l,p,w,g,b,d,c,f,S,v,k))},_.LeadFilter=Object.freeze({None:0,0:"None",Synchronize:1,1:"Synchronize"}),_.AbilityFilter=Object.freeze({Any:3,3:"Any",Ability0:0,0:"Ability0",Ability1:1,1:"Ability1"}),_.Ability=Object.freeze({Ability0:0,0:"Ability0",Ability1:1,1:"Ability1"}),_.NatureFilter=Object.freeze({Hardy:0,0:"Hardy",Lonely:1,1:"Lonely",Brave:2,2:"Brave",Adamant:3,3:"Adamant",Naughty:4,4:"Naughty",Bold:5,5:"Bold",Docile:6,6:"Docile",Relaxed:7,7:"Relaxed",Impish:8,8:"Impish",Lax:9,9:"Lax",Timid:10,10:"Timid",Hasty:11,11:"Hasty",Serious:12,12:"Serious",Jolly:13,13:"Jolly",Naive:14,14:"Naive",Modest:15,15:"Modest",Mild:16,16:"Mild",Quiet:17,17:"Quiet",Bashful:18,18:"Bashful",Rash:19,19:"Rash",Calm:20,20:"Calm",Gentle:21,21:"Gentle",Sassy:22,22:"Sassy",Careful:23,23:"Careful",Quirky:24,24:"Quirky",Any:25,25:"Any"}),_.Nature=Object.freeze({Hardy:0,0:"Hardy",Lonely:1,1:"Lonely",Brave:2,2:"Brave",Adamant:3,3:"Adamant",Naughty:4,4:"Naughty",Bold:5,5:"Bold",Docile:6,6:"Docile",Relaxed:7,7:"Relaxed",Impish:8,8:"Impish",Lax:9,9:"Lax",Timid:10,10:"Timid",Hasty:11,11:"Hasty",Serious:12,12:"Serious",Jolly:13,13:"Jolly",Naive:14,14:"Naive",Modest:15,15:"Modest",Mild:16,16:"Mild",Quiet:17,17:"Quiet",Bashful:18,18:"Bashful",Rash:19,19:"Rash",Calm:20,20:"Calm",Gentle:21,21:"Gentle",Sassy:22,22:"Sassy",Careful:23,23:"Careful",Quirky:24,24:"Quirky",Synchronize:25,25:"Synchronize"}),_.ShinyFilter=Object.freeze({None:0,0:"None",Star:1,1:"Star",Square:2,2:"Square",Both:3,3:"Both",Any:4,4:"Any"}),_.EncounterFilter=Object.freeze({Static:0,0:"Static",Dynamic:1,1:"Dynamic"}),_.Shiny=Object.freeze({None:0,0:"None",Star:1,1:"Star",Square:2,2:"Square",Both:3,3:"Both",All:4,4:"All"}),_.EncounterSlotFilter=Object.freeze({Any:12,12:"Any",Slot0:0,0:"Slot0",Slot1:1,1:"Slot1",Slot2:2,2:"Slot2",Slot3:3,3:"Slot3",Slot4:4,4:"Slot4",Slot5:5,5:"Slot5",Slot6:6,6:"Slot6",Slot7:7,7:"Slot7",Slot8:8,8:"Slot8",Slot9:9,9:"Slot9",Slot10:10,10:"Slot10",Slot11:11,11:"Slot11"}),_.EncounterSlot=Object.freeze({Slot0:0,0:"Slot0",Slot1:1,1:"Slot1",Slot2:2,2:"Slot2",Slot3:3,3:"Slot3",Slot4:4,4:"Slot4",Slot5:5,5:"Slot5",Slot6:6,6:"Slot6",Slot7:7,7:"Slot7",Slot8:8,8:"Slot8",Slot9:9,9:"Slot9",Slot10:10,10:"Slot10",Slot11:11,11:"Slot11"}),_.GenderRatio=Object.freeze({NoSetGender:256,256:"NoSetGender",Genderless:255,255:"Genderless",Male50Female50:127,127:"Male50Female50",Male25Female75:191,191:"Male25Female75",Male75Female25:63,63:"Male75Female25",Male875Female125:31,31:"Male875Female125",Male:0,0:"Male",Female:254,254:"Female"}),_.Gender=Object.freeze({Genderless:255,255:"Genderless",Male:0,0:"Male",Female:254,254:"Female"}),_.GenderFilter=Object.freeze({Any:256,256:"Any",Male:0,0:"Male",Female:254,254:"Female"});class x{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const e=this.__destroy_into_raw();t.__wbg_pokemon_free(e)}get shiny_value(){return t.__wbg_get_pokemon_shiny_value(this.ptr)>>>0}set shiny_value(e){t.__wbg_set_pokemon_shiny_value(this.ptr,e)}get pid(){return t.__wbg_get_pokemon_pid(this.ptr)>>>0}set pid(e){t.__wbg_set_pokemon_pid(this.ptr,e)}get ec(){return t.__wbg_get_pokemon_ec(this.ptr)>>>0}set ec(e){t.__wbg_set_pokemon_ec(this.ptr,e)}get nature(){return t.__wbg_get_pokemon_nature(this.ptr)>>>0}set nature(e){t.__wbg_set_pokemon_nature(this.ptr,e)}get ivs(){try{const s=t.__wbindgen_add_to_stack_pointer(-16);t.__wbg_get_pokemon_ivs(s,this.ptr);var e=d()[s/4+0],_=d()[s/4+1],r=j(e,_).slice();return t.__wbindgen_free(e,4*_),r}finally{t.__wbindgen_add_to_stack_pointer(16)}}set ivs(e){var _=B(e,t.__wbindgen_malloc),r=n;t.__wbg_set_pokemon_ivs(this.ptr,_,r)}get ability(){return t.__wbg_get_pokemon_ability(this.ptr)>>>0}set ability(e){t.__wbg_set_pokemon_ability(this.ptr,e)}get gender(){return t.__wbg_get_pokemon_gender(this.ptr)>>>0}set gender(e){t.__wbg_set_pokemon_gender(this.ptr,e)}get encounter(){return t.__wbg_get_pokemon_encounter(this.ptr)>>>0}set encounter(e){t.__wbg_set_pokemon_encounter(this.ptr,e)}get advances(){return t.__wbg_get_pokemon_advances(this.ptr)>>>0}set advances(e){t.__wbg_set_pokemon_advances(this.ptr,e)}get is_rare(){return 0!==t.__wbg_get_pokemon_is_rare(this.ptr)}set is_rare(e){t.__wbg_set_pokemon_is_rare(this.ptr,e)}}class z{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const e=this.__destroy_into_raw();t.__wbg_result_free(e)}get state0(){return t.__wbg_get_result_state0(this.ptr)>>>0}set state0(e){t.__wbg_set_result_state0(this.ptr,e)}get state1(){return t.__wbg_get_result_state1(this.ptr)>>>0}set state1(e){t.__wbg_set_result_state1(this.ptr,e)}get state2(){return t.__wbg_get_result_state2(this.ptr)>>>0}set state2(e){t.__wbg_set_result_state2(this.ptr,e)}get state3(){return t.__wbg_get_result_state3(this.ptr)>>>0}set state3(e){t.__wbg_set_result_state3(this.ptr,e)}get advances(){return t.__wbg_get_result_advances(this.ptr)>>>0}set advances(e){t.__wbg_set_result_advances(this.ptr,e)}get tid(){return t.__wbg_get_result_tid(this.ptr)}set tid(e){t.__wbg_set_result_tid(this.ptr,e)}get tsv(){return t.__wbg_get_result_tsv(this.ptr)}set tsv(e){t.__wbg_set_result_tsv(this.ptr,e)}get g8tid(){return t.__wbg_get_result_g8tid(this.ptr)>>>0}set g8tid(e){t.__wbg_set_result_g8tid(this.ptr,e)}get sid(){return t.__wbg_get_result_sid(this.ptr)}set sid(e){t.__wbg_set_result_sid(this.ptr,e)}get filter_type(){return m(t.__wbg_get_result_filter_type(this.ptr))}set filter_type(e){t.__wbg_set_result_filter_type(this.ptr,w(e))}}class O{static __wrap(t){const e=Object.create(O.prototype);return e.ptr=t,e}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const e=this.__destroy_into_raw();t.__wbg_shinyresult_free(e)}get state0(){try{const r=t.__wbindgen_add_to_stack_pointer(-16);t.__wbg_get_shinyresult_state0(r,this.ptr);var e=d()[r/4+0],_=d()[r/4+1];return A[0]=e,A[1]=_,M[0]}finally{t.__wbindgen_add_to_stack_pointer(16)}}set state0(e){M[0]=e;const _=A[0],r=A[1];t.__wbg_set_shinyresult_state0(this.ptr,_,r)}get state1(){try{const r=t.__wbindgen_add_to_stack_pointer(-16);t.__wbg_get_shinyresult_state1(r,this.ptr);var e=d()[r/4+0],_=d()[r/4+1];return A[0]=e,A[1]=_,M[0]}finally{t.__wbindgen_add_to_stack_pointer(16)}}set state1(e){M[0]=e;const _=A[0],r=A[1];t.__wbg_set_shinyresult_state1(this.ptr,_,r)}get advances(){return t.__wbg_get_shinyresult_advances(this.ptr)>>>0}set advances(e){t.__wbg_set_shinyresult_advances(this.ptr,e)}get shiny_value(){return t.__wbg_get_shinyresult_shiny_value(this.ptr)>>>0}set shiny_value(e){t.__wbg_set_shinyresult_shiny_value(this.ptr,e)}get ec(){return t.__wbg_get_shinyresult_ec(this.ptr)>>>0}set ec(e){t.__wbg_set_shinyresult_ec(this.ptr,e)}get pid(){return t.__wbg_get_shinyresult_pid(this.ptr)>>>0}set pid(e){t.__wbg_set_shinyresult_pid(this.ptr,e)}get nature(){return t.__wbg_get_shinyresult_nature(this.ptr)>>>0}set nature(e){t.__wbg_set_shinyresult_nature(this.ptr,e)}get ability(){return t.__wbg_get_shinyresult_ability(this.ptr)>>>0}set ability(e){t.__wbg_set_shinyresult_ability(this.ptr,e)}}class G{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const e=this.__destroy_into_raw();t.__wbg_shinyresultbdspstationary_free(e)}get state0(){return t.__wbg_get_shinyresultbdspstationary_state0(this.ptr)>>>0}set state0(e){t.__wbg_set_shinyresultbdspstationary_state0(this.ptr,e)}get state1(){return t.__wbg_get_shinyresultbdspstationary_state1(this.ptr)>>>0}set state1(e){t.__wbg_set_shinyresultbdspstationary_state1(this.ptr,e)}get state2(){return t.__wbg_get_shinyresultbdspstationary_state2(this.ptr)>>>0}set state2(e){t.__wbg_set_shinyresultbdspstationary_state2(this.ptr,e)}get state3(){return t.__wbg_get_shinyresultbdspstationary_state3(this.ptr)>>>0}set state3(e){t.__wbg_set_shinyresultbdspstationary_state3(this.ptr,e)}get advances(){return t.__wbg_get_shinyresultbdspstationary_advances(this.ptr)>>>0}set advances(e){t.__wbg_set_shinyresultbdspstationary_advances(this.ptr,e)}get shiny_value(){return t.__wbg_get_shinyresultbdspstationary_shiny_value(this.ptr)>>>0}set shiny_value(e){t.__wbg_set_shinyresultbdspstationary_shiny_value(this.ptr,e)}get pid(){return t.__wbg_get_shinyresultbdspstationary_pid(this.ptr)>>>0}set pid(e){t.__wbg_set_shinyresultbdspstationary_pid(this.ptr,e)}get ec(){return t.__wbg_get_shinyresultbdspstationary_ec(this.ptr)>>>0}set ec(e){t.__wbg_set_shinyresultbdspstationary_ec(this.ptr,e)}get nature(){return t.__wbg_get_shinyresultbdspstationary_nature(this.ptr)>>>0}set nature(e){t.__wbg_set_shinyresultbdspstationary_nature(this.ptr,e)}get ivs(){try{const s=t.__wbindgen_add_to_stack_pointer(-16);t.__wbg_get_shinyresultbdspstationary_ivs(s,this.ptr);var e=d()[s/4+0],_=d()[s/4+1],r=j(e,_).slice();return t.__wbindgen_free(e,4*_),r}finally{t.__wbindgen_add_to_stack_pointer(16)}}set ivs(e){var _=B(e,t.__wbindgen_malloc),r=n;t.__wbg_set_shinyresultbdspstationary_ivs(this.ptr,_,r)}get ability(){return t.__wbg_get_shinyresultbdspstationary_ability(this.ptr)>>>0}set ability(e){t.__wbg_set_shinyresultbdspstationary_ability(this.ptr,e)}get gender(){return t.__wbg_get_shinyresultbdspstationary_gender(this.ptr)>>>0}set gender(e){t.__wbg_set_shinyresultbdspstationary_gender(this.ptr,e)}}class R{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const e=this.__destroy_into_raw();t.__wbg_xoroshiro_free(e)}}class T{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const e=this.__destroy_into_raw();t.__wbg_xorshift_free(e)}}function C(e){const r=fetch(e);let s;return(s="function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(r,{"./wasm_bg.js":_}):r.then(t=>t.arrayBuffer()).then(t=>WebAssembly.instantiate(t,{"./wasm_bg.js":_}))).then(({instance:e})=>{t=C.wasm=e.exports,_.wasm=t})}function D(r){const s=require("fs");return new Promise(function(t,_){s.readFile(e+r,function(e,r){e?_(e):t(r.buffer)})}).then(t=>WebAssembly.instantiate(t,{"./wasm_bg":_})).then(({instance:e})=>{t=C.wasm=e.exports,_.wasm=t})}_.__wbindgen_json_serialize=function(e,_){const r=s(_);var i=g(JSON.stringify(void 0===r?null:r),t.__wbindgen_malloc,t.__wbindgen_realloc),a=n;d()[e/4+1]=a,d()[e/4+0]=i},_.__wbindgen_json_parse=function(t,e){return w(JSON.parse(y(t,e)))},_.__wbg_new_949bbc1147195c4e=function(){return w(new Array)},_.__wbg_shinyresult_new=function(t){return w(O.__wrap(t))},_.__wbg_push_284486ca27c6aa8b=function(t,e){return s(t).push(s(e))},_.__wbindgen_object_drop_ref=function(t){m(t)},_.__wbg_new_693216e109162396=function(){return w(new Error)},_.__wbg_stack_0ddaca5d1abfb52f=function(e,_){var r=g(s(_).stack,t.__wbindgen_malloc,t.__wbindgen_realloc),i=n;d()[e/4+1]=i,d()[e/4+0]=r},_.__wbg_error_09919627ac0992f5=function(e,_){try{console.error(y(e,_))}finally{t.__wbindgen_free(e,_)}},_.__wbindgen_string_new=function(t,e){return w(y(t,e))},_.__wbindgen_string_get=function(e,_){const r=s(_);var i="string"==typeof r?r:void 0,a=S(i)?0:g(i,t.__wbindgen_malloc,t.__wbindgen_realloc),o=n;d()[e/4+1]=o,d()[e/4+0]=a},_.__wbindgen_throw=function(t,e){throw new Error(y(t,e))},_.Pokemon=x,_.Result=z,_.ShinyResult=O,_.ShinyResultBdspStationary=G,_.Xoroshiro=R,_.Xorshift=T;const E=Object.assign(C,_);module.exports=function(t){return E(t).then(()=>_)};
},{"fs":"rDCW"}],0:[function(require,module,exports) {
var b=require("TUK3");b.register("wasm",require("fISM"));b.load([["wasm_bg.9b1f8779.wasm","lGJG"]]).then(function(){require("njwa");});
},{}]},{},[0], null)
//# sourceMappingURL=/getResults.b807677b.js.map