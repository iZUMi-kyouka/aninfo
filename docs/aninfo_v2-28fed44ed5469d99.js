let a4=`Object`,V=null,Y=`undefined`,ab=772,a2=`boolean`,$=`function`,a7=4,a1=`number`,T=128,ac=1420,W=0,a3=`string`,Z=`utf-8`,a0=1,S=Array,aa=Date,_=Error,a6=FinalizationRegistry,a5=JSON.stringify,a9=Object,a8=Reflect,X=Uint8Array,U=undefined;var u=(a=>{const b=typeof a;if(b==a1||b==a2||a==V){return `${a}`};if(b==a3){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==V){return `Symbol`}else{return `Symbol(${b})`}};if(b==$){const b=a.name;if(typeof b==a3&&b.length>W){return `Function(${b})`}else{return `Function`}};if(S.isArray(a)){const b=a.length;let c=`[`;if(b>W){c+=u(a[W])};for(let d=a0;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>a0){d=c[a0]}else{return toString.call(a)};if(d==a4){try{return `Object(`+ a5(a)+ `)`}catch(a){return a4}};if(a instanceof _){return `${a.name}: ${a.message}\n${a.stack}`};return d});var z=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h70e7ff057ef1b373(c,d,y(e))}finally{b[x++]=U}});var O=((a,b)=>{});function I(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(q(b))}}var r=(a=>a===U||a===V);var c=(a=>b[a]);var E=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h1406b086444884f0(c,d,y(e))}finally{b[x++]=U}});var F=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__hced98bdb65514a8e(b,c)});var t=(()=>{if(s===V||s.byteLength===W){s=new Float64Array(a.memory.buffer)};return s});var N=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_json_serialize=((b,d)=>{const e=c(d);const f=a5(e===U?V:e);const h=l(f,a.__wbindgen_malloc,a.__wbindgen_realloc);const i=g;n()[b/a7+ a0]=i;n()[b/a7+ W]=h});b.wbg.__wbindgen_string_new=((a,b)=>{const c=p(a,b);return q(c)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==a0){b.a=W;return !0};const c=!1;return c});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return q(b)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===U;return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return q(b)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===a1?d:U;t()[a/8+ a0]=r(e)?W:e;n()[a/a7+ W]=!r(e)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===a3;return b});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===a3?e:U;var h=r(f)?W:l(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=g;n()[b/a7+ a0]=i;n()[b/a7+ W]=h});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===$;return b});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;n()[a/a7+ a0]=r(d)?W:d;n()[a/a7+ W]=!r(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>W});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;n()[a/a7+ a0]=r(d)?W:d;n()[a/a7+ W]=!r(d)});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>W});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>W});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;n()[a/a7+ a0]=r(d)?W:d;n()[a/a7+ W]=!r(d)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new _();return q(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(p(b,c))}finally{a.__wbindgen_free(d,e,a0)}});b.wbg.__wbg_clearTimeout_541ac0980ffcef74=(a=>{const b=clearTimeout(f(a));return q(b)});b.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return I(((a,b)=>{const d=setTimeout(c(a),b);return q(d)}),arguments)};b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return q(b)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==V;return d});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new _(p(a,b));return q(c)});b.wbg.__wbg_crypto_d05b68a3572bb8ca=(a=>{const b=c(a).crypto;return q(b)});b.wbg.__wbg_process_b02b3570280d0366=(a=>{const b=c(a).process;return q(b)});b.wbg.__wbg_versions_c1cb42213cedf0f5=(a=>{const b=c(a).versions;return q(b)});b.wbg.__wbg_node_43b1089f407e4ec2=(a=>{const b=c(a).node;return q(b)});b.wbg.__wbg_msCrypto_10fc94afee92bd76=(a=>{const b=c(a).msCrypto;return q(b)});b.wbg.__wbg_require_9a7e0f667ead4995=function(){return I((()=>{const a=module.require;return q(a)}),arguments)};b.wbg.__wbg_randomFillSync_b70ccbdf4926a99d=function(){return I(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d=function(){return I(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===a2?(b?a0:W):2;return d});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const d=c(a)[c(b)];return q(d)});b.wbg.__wbg_set_f975102236d3c502=((a,b,d)=>{c(a)[f(b)]=f(d)});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=L(b,c).slice();a.__wbindgen_free(b,c*a7,a7);console.error(...d)});b.wbg.__wbg_log_7c3433e130418e14=((b,c)=>{var d=L(b,c).slice();a.__wbindgen_free(b,c*a7,a7);console.log(...d)});b.wbg.__wbg_title_ae59cbd3351df0d9=((b,d)=>{const e=c(d).title;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_settitle_33da3ee844c607ef=((a,b,d)=>{c(a).title=p(b,d)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return r(b)?W:q(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return I(((a,b,d)=>{const e=c(a).createElement(p(b,d));return q(e)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return I(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===W?U:p(b,d),p(e,f));return q(g)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{const e=c(a).createTextNode(p(b,d));return q(e)});b.wbg.__wbg_getElementById_c369ff43f0db99cf=((a,b,d)=>{const e=c(a).getElementById(p(b,d));return r(e)?W:q(e)});b.wbg.__wbg_getElementsByClassName_140edb1da19cbc37=((a,b,d)=>{const e=c(a).getElementsByClassName(p(b,d));return q(e)});b.wbg.__wbg_getElementsByTagName_7d0ffbf0086bbbd1=((a,b,d)=>{const e=c(a).getElementsByTagName(p(b,d));return q(e)});b.wbg.__wbg_querySelector_a5f74efc5fa193dd=function(){return I(((a,b,d)=>{const e=c(a).querySelector(p(b,d));return r(e)?W:q(e)}),arguments)};b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return r(b)?W:q(b)});b.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=c(a).location;return q(b)});b.wbg.__wbg_history_bc4057de66a2015f=function(){return I((a=>{const b=c(a).history;return q(b)}),arguments)};b.wbg.__wbg_innerWidth_758af301ca4baa3e=function(){return I((a=>{const b=c(a).innerWidth;return q(b)}),arguments)};b.wbg.__wbg_innerHeight_c1ef73925c3d3e9c=function(){return I((a=>{const b=c(a).innerHeight;return q(b)}),arguments)};b.wbg.__wbg_alert_820a3ec9534dea89=function(){return I(((a,b,d)=>{c(a).alert(p(b,d))}),arguments)};b.wbg.__wbg_confirm_5c66cecc6cf673d1=function(){return I(((a,b,d)=>{const e=c(a).confirm(p(b,d));return e}),arguments)};b.wbg.__wbg_scrollTo_4d970c5e1c4b340b=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbg_cancelAnimationFrame_111532f326e480af=function(){return I(((a,b)=>{c(a).cancelAnimationFrame(b)}),arguments)};b.wbg.__wbg_requestAnimationFrame_549258cfa66011f0=function(){return I(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_fetch_c4b6afebdb1f918e=((a,b)=>{const d=c(a).fetch(c(b));return q(d)});b.wbg.__wbg_instanceof_Element_6945fc210db80ea9=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_5235ee79fd5f6781=((b,d)=>{const e=c(d).namespaceURI;var f=r(e)?W:l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{c(a).innerHTML=p(b,d)});b.wbg.__wbg_outerHTML_e073aa84e7bc1eaf=((b,d)=>{const e=c(d).outerHTML;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_getAttribute_99bddb29274b29b9=((b,d,e,f)=>{const h=c(d).getAttribute(p(e,f));var i=r(h)?W:l(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=g;n()[b/a7+ a0]=j;n()[b/a7+ W]=i});b.wbg.__wbg_getBoundingClientRect_91e6d57c4e65f745=(a=>{const b=c(a).getBoundingClientRect();return q(b)});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return I(((a,b,d)=>{c(a).removeAttribute(p(b,d))}),arguments)};b.wbg.__wbg_scrollBy_256f11fcd3546288=((a,b,d)=>{c(a).scrollBy(b,d)});b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return I(((a,b,d,e,f)=>{c(a).setAttribute(p(b,d),p(e,f))}),arguments)};b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return I(((a,b,d,e,f)=>{c(a).addEventListener(p(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_5d31483804421bfa=function(){return I(((a,b,d,e,f)=>{c(a).removeEventListener(p(b,d),c(e),f!==W)}),arguments)};b.wbg.__wbg_length_07fa9cc13a1c3bba=function(){return I((a=>{const b=c(a).length;return b}),arguments)};b.wbg.__wbg_state_9cc3f933b7d50acb=function(){return I((a=>{const b=c(a).state;return q(b)}),arguments)};b.wbg.__wbg_go_ecdcadf4d98a07a6=function(){return I(((a,b)=>{c(a).go(b)}),arguments)};b.wbg.__wbg_pushState_b8e8d346f8bb33fd=function(){return I(((a,b,d,e,f,g)=>{c(a).pushState(c(b),p(d,e),f===W?U:p(f,g))}),arguments)};b.wbg.__wbg_replaceState_ec9431bea5108a50=function(){return I(((a,b,d,e,f,g)=>{c(a).replaceState(c(b),p(d,e),f===W?U:p(f,g))}),arguments)};b.wbg.__wbg_length_c2d5de977172fa1a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_getwithindex_6f132a79d56e81ec=((a,b)=>{const d=c(a)[b>>>W];return r(d)?W:q(d)});b.wbg.__wbg_url_7807f6a1fddc3e23=((b,d)=>{const e=c(d).url;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_newwithstr_36b0b3f97efe096f=function(){return I(((a,b)=>{const c=new Request(p(a,b));return q(c)}),arguments)};b.wbg.__wbg_newwithstrandinit_3fd6fba4083ff2d0=function(){return I(((a,b,d)=>{const e=new Request(p(a,b),c(d));return q(e)}),arguments)};b.wbg.__wbg_instanceof_Response_849eb93e75734b6e=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_headers_9620bfada380764a=(a=>{const b=c(a).headers;return q(b)});b.wbg.__wbg_json_1d5f113e916d8675=function(){return I((a=>{const b=c(a).json();return q(b)}),arguments)};b.wbg.__wbg_text_450a059667fd91fd=function(){return I((a=>{const b=c(a).text();return q(b)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return q(b)});b.wbg.__wbg_href_7bfb3b2fdc0a6c3f=((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_pathname_c5fe403ef9525ec6=((b,d)=>{const e=c(d).pathname;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_search_c68f506c44be6d1e=((b,d)=>{const e=c(d).search;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_setsearch_fd62f4de409a2bb3=((a,b,d)=>{c(a).search=p(b,d)});b.wbg.__wbg_hash_cdea7a9b7e684a42=((b,d)=>{const e=c(d).hash;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_sethash_9bacb48849d0016e=((a,b,d)=>{c(a).hash=p(b,d)});b.wbg.__wbg_new_67853c351755d2cf=function(){return I(((a,b)=>{const c=new URL(p(a,b));return q(c)}),arguments)};b.wbg.__wbg_newwithbase_6aabbfb1b2e6a1cb=function(){return I(((a,b,c,d)=>{const e=new URL(p(a,b),p(c,d));return q(e)}),arguments)};b.wbg.__wbg_new_ab6fd82b10560829=function(){return I((()=>{const a=new Headers();return q(a)}),arguments)};b.wbg.__wbg_get_0ebaad3318b38f2a=function(){return I(((b,d,e,f)=>{const h=c(d).get(p(e,f));var i=r(h)?W:l(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=g;n()[b/a7+ a0]=j;n()[b/a7+ W]=i}),arguments)};b.wbg.__wbg_set_cb0e7a5c2dd66afd=function(){return I(((a,b,d,e,f)=>{c(a).set(p(b,d),p(e,f))}),arguments)};b.wbg.__wbg_instanceof_Node_daad148a35d38074=(a=>{let b;try{b=c(a) instanceof Node}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return r(b)?W:q(b)});b.wbg.__wbg_parentElement_347524db59fc2976=(a=>{const b=c(a).parentElement;return r(b)?W:q(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return q(b)});b.wbg.__wbg_lastChild_83efe6d5da370e1f=(a=>{const b=c(a).lastChild;return r(b)?W:q(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return r(b)?W:q(b)});b.wbg.__wbg_setnodeValue_94b86af0cda24b90=((a,b,d)=>{c(a).nodeValue=b===W?U:p(b,d)});b.wbg.__wbg_textContent_8e392d624539e731=((b,d)=>{const e=c(d).textContent;var f=r(e)?W:l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return I((a=>{const b=c(a).cloneNode();return q(b)}),arguments)};b.wbg.__wbg_contains_fdfd1dc667f36695=((a,b)=>{const d=c(a).contains(c(b));return d});b.wbg.__wbg_insertBefore_d2a001abf538c1f8=function(){return I(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return q(e)}),arguments)};b.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return I(((a,b)=>{const d=c(a).removeChild(c(b));return q(d)}),arguments)};b.wbg.__wbg_instanceof_WorkerGlobalScope_46b577f151fad960=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_fetch_921fad6ef9e883dd=((a,b)=>{const d=c(a).fetch(c(b));return q(d)});b.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=c(a).target;return r(b)?W:q(b)});b.wbg.__wbg_bubbles_abce839854481bc6=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return q(b)});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_new_4c501d7c115d20a6=function(){return I((()=>{const a=new URLSearchParams();return q(a)}),arguments)};b.wbg.__wbg_instanceof_DomRectReadOnly_1466e21123d3d6e2=(a=>{let b;try{b=c(a) instanceof DOMRectReadOnly}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_left_fe0a839abdd508f4=(a=>{const b=c(a).left;return b});b.wbg.__wbg_href_2edbae9e92cdfeff=((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_instanceof_HtmlInputElement_307512fe1252c849=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_setchecked_931ff2ed2cd3ebfd=((a,b)=>{c(a).checked=b!==W});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_setvalue_78cb4f1fef58ae98=((a,b,d)=>{c(a).value=p(b,d)});b.wbg.__wbg_href_706b235ecfe6848c=function(){return I(((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f}),arguments)};b.wbg.__wbg_pathname_5449afe3829f96a1=function(){return I(((b,d)=>{const e=c(d).pathname;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f}),arguments)};b.wbg.__wbg_search_489f12953342ec1f=function(){return I(((b,d)=>{const e=c(d).search;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f}),arguments)};b.wbg.__wbg_hash_553098e838e06c1d=function(){return I(((b,d)=>{const e=c(d).hash;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f}),arguments)};b.wbg.__wbg_instanceof_HtmlDialogElement_2889781168e1a92e=(a=>{let b;try{b=c(a) instanceof HTMLDialogElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_close_6442eced49eaa9dc=(a=>{c(a).close()});b.wbg.__wbg_show_73ba0aff7eb6ee64=(a=>{c(a).show()});b.wbg.__wbg_value_d7f5bfbd9302c14b=((b,d)=>{const e=c(d).value;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbg_setvalue_090972231f0a4f6f=((a,b,d)=>{c(a).value=p(b,d)});b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>W];return q(d)});b.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{const c=new Function(p(a,b));return q(c)});b.wbg.__wbg_get_e3c254076557e348=function(){return I(((a,b)=>{const d=a8.get(c(a),c(b));return q(d)}),arguments)};b.wbg.__wbg_call_27c0f87801dedf93=function(){return I(((a,b)=>{const d=c(a).call(c(b));return q(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a9();return q(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return I((()=>{const a=self.self;return q(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return I((()=>{const a=window.window;return q(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return I((()=>{const a=globalThis.globalThis;return q(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return I((()=>{const a=global.global;return q(a)}),arguments)};b.wbg.__wbg_from_89e3fc3ba5e6fb48=(a=>{const b=S.from(c(a));return q(b)});b.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_Error_e20bb56fd5591a93=(a=>{let b;try{b=c(a) instanceof _}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_message_5bf28016c2b49cfb=(a=>{const b=c(a).message;return q(b)});b.wbg.__wbg_name_e7429f0dda6079e2=(a=>{const b=c(a).name;return q(b)});b.wbg.__wbg_toString_ffe4c9ea3b3532e9=(a=>{const b=c(a).toString();return q(b)});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return I(((a,b,d)=>{const e=c(a).call(c(b),c(d));return q(e)}),arguments)};b.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_getTime_2bc4375165f02d15=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_getTimezoneOffset_38257122e236c190=(a=>{const b=c(a).getTimezoneOffset();return b});b.wbg.__wbg_new_cf3ec55744a78578=(a=>{const b=new aa(c(a));return q(b)});b.wbg.__wbg_new0_7d84e5b2cd9fdc73=(()=>{const a=new aa();return q(a)});b.wbg.__wbg_entries_95cc2c823b285a09=(a=>{const b=a9.entries(c(a));return q(b)});b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=a9.is(c(a),c(b));return d});b.wbg.__wbg_toString_c816a20ab859d0c1=(a=>{const b=c(a).toString();return q(b)});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(c(a));return q(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return q(d)});b.wbg.__wbg_then_a73caa9a87991566=((a,b,d)=>{const e=c(a).then(c(b),c(d));return q(e)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return q(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const e=new X(c(a),b>>>W,d>>>W);return q(e)});b.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new X(c(a));return q(b)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>W)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6=(a=>{let b;try{b=c(a) instanceof X}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new X(a>>>W);return q(b)});b.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,d)=>{const e=c(a).subarray(b>>>W,d>>>W);return q(e)});b.wbg.__wbg_set_1f9b04f170055d33=function(){return I(((a,b,d)=>{const e=a8.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a7+ a0]=h;n()[b/a7+ W]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new _(p(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return q(b)});b.wbg.__wbindgen_closure_wrapper1292=((a,b,c)=>{const d=w(a,b,ab,z);return q(d)});b.wbg.__wbindgen_closure_wrapper1294=((a,b,c)=>{const d=w(a,b,ab,A);return q(d)});b.wbg.__wbindgen_closure_wrapper2566=((a,b,c)=>{const d=w(a,b,ac,B);return q(d)});b.wbg.__wbindgen_closure_wrapper2568=((a,b,c)=>{const d=C(a,b,ac,D);return q(d)});b.wbg.__wbindgen_closure_wrapper3145=((a,b,c)=>{const d=C(a,b,1639,E);return q(d)});b.wbg.__wbindgen_closure_wrapper3347=((a,b,c)=>{const d=w(a,b,1722,F);return q(d)});b.wbg.__wbindgen_closure_wrapper3370=((a,b,c)=>{const d=w(a,b,1728,G);return q(d)});b.wbg.__wbindgen_closure_wrapper3443=((a,b,c)=>{const d=w(a,b,1755,H);return q(d)});return b});var K=(()=>{if(J===V||J.byteLength===W){J=new Uint32Array(a.memory.buffer)};return J});var l=((a,b,c)=>{if(c===U){const c=j.encode(a);const d=b(c.length,a0)>>>W;i().subarray(d,d+ c.length).set(c);g=c.length;return d};let d=a.length;let e=b(d,a0)>>>W;const f=i();let h=W;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;f[e+ h]=b};if(h!==d){if(h!==W){a=a.slice(h)};e=c(e,d,d=h+ a.length*3,a0)>>>W;const b=i().subarray(e+ h,e+ d);const f=k(a,b);h+=f.written;e=c(e,d,h,a0)>>>W};g=h;return e});var i=(()=>{if(h===V||h.byteLength===W){h=new X(a.memory.buffer)};return h});var D=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1__h08d066415bbf422d(b,c,q(d))});var M=(async(a,b)=>{if(typeof Response===$&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===$){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var H=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h74ac44615456ae2d(c,d,y(e))}finally{b[x++]=U}});var L=((a,b)=>{a=a>>>W;const c=K();const d=c.subarray(a/a7,a/a7+ b);const e=[];for(let a=W;a<d.length;a++){e.push(f(d[a]))};return e});var q=(a=>{if(d===b.length)b.push(b.length+ a0);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var n=(()=>{if(m===V||m.byteLength===W){m=new Int32Array(a.memory.buffer)};return m});var R=(async(b)=>{if(a!==U)return a;if(typeof b===Y){b=new URL(`aninfo_v2-28fed44ed5469d99_bg.wasm`,import.meta.url)};const c=N();if(typeof b===a3||typeof Request===$&&b instanceof Request||typeof URL===$&&b instanceof URL){b=fetch(b)};O(c);const {instance:d,module:e}=await M(await b,c);return P(d,e)});var Q=(b=>{if(a!==U)return a;const c=N();O(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return P(d,b)});var P=((b,c)=>{a=b.exports;R.__wbindgen_wasm_module=c;s=V;m=V;J=V;h=V;a.__wbindgen_start();return a});var G=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc359a4a9df669012(b,c,q(d))});var C=((b,c,d,e)=>{const f={a:b,b:c,cnt:a0,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===W){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=W;v.unregister(f)}}};g.original=f;v.register(g,f,f);return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var B=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h42a448b1bdb2f038(c,d,y(e))}finally{b[x++]=U}});var A=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__hfa697bd6222a2b18(b,c)});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:a0,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=W;try{return e(c,f.b,...b)}finally{if(--f.cnt===W){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var p=((a,b)=>{a=a>>>W;return o.decode(i().subarray(a,a+ b))});var y=(a=>{if(x==a0)throw new _(`out of js stack`);b[--x]=a;return x});let a;const b=new S(T).fill(U);b.push(U,V,!0,!1);let d=b.length;let g=W;let h=V;const j=typeof TextEncoder!==Y?new TextEncoder(Z):{encode:()=>{throw _(`TextEncoder not available`)}};const k=typeof j.encodeInto===$?((a,b)=>j.encodeInto(a,b)):((a,b)=>{const c=j.encode(a);b.set(c);return {read:a.length,written:c.length}});let m=V;const o=typeof TextDecoder!==Y?new TextDecoder(Z,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw _(`TextDecoder not available`)}};if(typeof TextDecoder!==Y){o.decode()};let s=V;const v=typeof a6===Y?{register:()=>{},unregister:()=>{}}:new a6(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let x=T;let J=V;export default R;export{Q as initSync}