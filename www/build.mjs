import { writeFile, mkdir } from 'node:fs/promises';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { renderFrameToSvg, validatePixelArt } from '@openpixel/renderer';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const out=resolve(root,'www/assets/landscape');
const w=96,h=54;
const C={sky:'#253c4a',dusk:'#49665c',far:'#71836a',snow:'#c4c69b',hill:'#354f49',pine:'#203d3c',deep:'#182f35',water:'#284b50',shine:'#6b8981',glow:'#f2c879',reed:'#82915c',ink:'#162a31'};
const blank=()=>Array.from({length:h},()=>Array(w).fill(null));
const put=(m,x,y,c)=>{if(x>=0&&x<w&&y>=0&&y<h)m[y][x]=c;};
const row=(m,y,a,b,c)=>{for(let x=a;x<=b;x++)put(m,x,y,c);};
const fill=(m,x,y,ww,hh,c)=>{for(let yy=y;yy<y+hh;yy++)row(m,yy,x,x+ww-1,c);};
const ids=['sky','sun','distant','mountains','forest','lake','foreground','sparkles'];
const L=Object.fromEntries(ids.map(k=>[k,blank()]));
for(let y=0;y<h;y++)row(L.sky,y,0,w-1,y<16?C.sky:y<23?C.dusk:C.deep);
for(const [x,y,ww] of [[12,9,9],[56,6,12],[75,13,7]]){row(L.sky,y,x+2,x+ww-3,C.far);row(L.sky,y+1,x,x+ww-1,C.far);row(L.sky,y+2,x+1,x+ww-2,C.far);}
fill(L.sun,73,14,6,6,C.glow);fill(L.sun,71,16,10,2,C.glow);
for(let y=14;y<29;y++){const edge=43-Math.abs(y-22)*2;row(L.distant,y,0,edge,C.far);row(L.distant,y,95-edge,95,C.far);}
for(const [x,peak] of [[24,14],[72,15]])for(let dy=0;dy<8;dy++)row(L.distant,peak+dy,x-dy,x+dy,dy<3?C.snow:C.far);
for(let y=20;y<34;y++){const edge=28-Math.abs(y-26)*2;row(L.mountains,y,0,edge,C.hill);row(L.mountains,y,95-edge,95,C.hill);}
const tree=(x,base,s,c)=>{for(let lev=0;lev<s;lev++){const half=Math.floor(lev/2);row(L.forest,base-s+lev,x-half,x+half,c);}fill(L.forest,x-1,base-1,3,3,C.ink);};
for(let x=1;x<37;x+=5)tree(x,34,8+x%3,x%2?C.pine:C.hill);
for(let x=59;x<96;x+=5)tree(x,35,9+x%3,x%2?C.pine:C.hill);
fill(L.lake,0,31,w,23,C.water);
for(let y=34;y<h;y++){row(L.foreground,y,0,10+y%4,C.ink);row(L.foreground,y,85-y%4,95,C.ink);}
for(let x=0;x<35;x+=4){row(L.foreground,31+x%3,x,x+3,C.reed);put(L.foreground,x+1,30,C.reed);}
for(let x=62;x<96;x+=4){row(L.foreground,32+x%2,x,x+3,C.reed);put(L.foreground,x+2,31,C.reed);}
for(let x=8;x<26;x+=6){put(L.foreground,x,32,C.reed);put(L.foreground,x+1,30,C.reed);}
for(let x=72;x<89;x+=5){put(L.foreground,x,33,C.reed);put(L.foreground,x-1,31,C.reed);}
const frame=p=>ids.map(id=>{const pixels=L[id].map(r=>[...r]);if(id==='lake'){for(let i=0;i<9;i++){const x=(i*13+p*7)%86+4,y=35+(i*5+p*3)%16;row(pixels,y,x,x+i%3+1,(i+p)%2?C.shine:C.dusk);}}if(id==='sparkles'){for(const [x,y] of [[11,28],[32,36],[52,30],[66,40],[86,26],[44,46]])put(pixels,x+(p+x)%3-1,y,C.glow);}if(id==='sky'&&p){const base=L.sky;for(let y=0;y<h;y++)for(let x=0;x<w;x++)if(base[y][x]===C.far)pixels[y][x]=C.sky;for(let y=0;y<h;y++)for(let x=0;x<w;x++)if(base[y][x]===C.far)pixels[y][(x+p*2)%w]=C.far;}return{layerId:id,pixels};});
const asset={version:1,name:'Dusk over the quiet lake',width:w,height:h,loop:true,layers:ids.map((id,i)=>({id,name:['Evening sky','Last light','Blue mountains','Ridge','Pine forest','Moving water','Near shore','Fireflies'][i],visible:true,opacity:1})),frames:[0,1,2,3].map(p=>({durationMs:[900,700,1000,800][p],layers:frame(p)}))};
validatePixelArt(asset);await mkdir(out,{recursive:true});await writeFile(resolve(out,'landscape.json'),JSON.stringify(asset,null,2)+'\n','utf8');
for(let i=0;i<4;i++)await writeFile(resolve(out,`frame-${i+1}.svg`),renderFrameToSvg(asset,{frameIndex:i,pixelSize:12,title:'OpenPixel pixel art mountain lake'}),'utf8');
console.log(`Rendered ${w}×${h} pixel-art landscape (4 frames)`);

