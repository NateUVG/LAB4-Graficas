import bpy, bmesh
from mathutils import Vector

for obj in list(bpy.data.objects):
    if obj.name != "Cube":
        bpy.data.objects.remove(obj, do_unlink=True)
if "Cube" not in bpy.data.objects:
    bpy.ops.mesh.primitive_cube_add()
cubo = bpy.data.objects["Cube"]
cubo.name = "Nave"
cubo.location = (0, 0, 0)

def material(nombre, color):
    m = bpy.data.materials.new(nombre)
    m.use_nodes = True
    m.node_tree.nodes["Principled BSDF"].inputs["Base Color"].default_value = (*color, 1)
    m.diffuse_color = (*color, 1)
    cubo.data.materials.append(m)
    return len(cubo.data.materials) - 1

CASCO = material("Casco", (0.55, 0.58, 0.65))
ALAS = material("Alas", (0.85, 0.18, 0.12))
CABINA = material("Cabina", (0.15, 0.55, 0.95))
MOTOR = material("Motor", (1.0, 0.55, 0.05))

bm = bmesh.new()
bm.from_mesh(cubo.data)

def cara(normal, cerca=None):
    n = Vector(normal)
    candidatas = [f for f in bm.faces if f.normal.dot(n) > 0.99]
    if cerca is not None:
        candidatas.sort(key=lambda f: (f.calc_center_median() - Vector(cerca)).length)
    else:
        candidatas.sort(key=lambda f: -f.calc_center_median().dot(n))
    return candidatas[0]

def extruir(f, desplazamiento, escala=(1, 1, 1)):
    r = bmesh.ops.extrude_face_region(bm, geom=[f])
    verts = [e for e in r["geom"] if isinstance(e, bmesh.types.BMVert)]
    nueva = [e for e in r["geom"] if isinstance(e, bmesh.types.BMFace)][0]
    bmesh.ops.translate(bm, verts=verts, vec=Vector(desplazamiento))
    centro = nueva.calc_center_median()
    bmesh.ops.scale(bm, verts=verts, vec=Vector(escala), space=__import__("mathutils").Matrix.Translation(-centro))
    for b in bm.faces: b.normal_update()
    return nueva

# Scale: cuerpo central 
bmesh.ops.scale(bm, verts=bm.verts, vec=Vector((1.5, 0.8, 0.5)))
for f in bm.faces: f.material_index = CASCO

# baja hacia el frente.
frente = cara((1, 0, 0))
nariz = extruir(frente, (1.9, 0, -0.22), (1, 0.85, 0.3))

# Escape: inset atras y se hunde.
atras = cara((-1, 0, 0))
bmesh.ops.inset_region(bm, faces=[atras], thickness=0.15, depth=-0.25)
for f in bm.faces:
    if f.calc_center_median().x < -1.45:
        f.material_index = MOTOR

# Gondolas a los costados: brazo corto, pared grande y la gondola hacia afuera.
for signo in (1, -1):
    lado = cara((0, signo, 0))
    brazo = extruir(lado, (-0.5, signo * 0.5, 0), (0.5, 1, 0.6))
    pared = extruir(brazo, (0, signo * 0.05, 0.05), (1.9, 1, 2.0))
    gondola = extruir(pared, (0, signo * 1.1, 0), (1, 1, 0.85))
    bmesh.ops.inset_region(bm, faces=[gondola], thickness=0.12, depth=-0.12)
    for f in bm.faces:
        c = f.calc_center_median()
        if abs(c.y) > 2.3 and f.normal.dot(Vector((0, signo, 0))) > 0.9:
            f.material_index = MOTOR
    # Aleta chica arriba de cada gondola, inclinada hacia atras.
    tope = cara((0, 0, 1), (-1.0, signo * 1.9, 0.65))
    bmesh.ops.inset_region(bm, faces=[tope], thickness=0.3, depth=0)
    tope = cara((0, 0, 1), (-1.0, signo * 1.9, 0.65))
    aleta = extruir(tope, (-0.6, 0, 1.0), (0.3, 0.2, 1))
    aleta.material_index = ALAS
    for f in bm.faces:
        c = f.calc_center_median()
        if abs(c.y) > 1.5 and c.z > 0.75:
            f.material_index = ALAS

# Aleta central alta y delgada, inclinada hacia atras.
arriba = cara((0, 0, 1), (0, 0, 0.5))
bmesh.ops.inset_region(bm, faces=[arriba], thickness=0.45, depth=0)
arriba = cara((0, 0, 1), (0, 0, 0.5))
centro = extruir(arriba, (-0.5, 0, 1.0), (0.55, 0.25, 1))
punta = extruir(centro, (-0.8, 0, 1.3), (0.5, 0.7, 1))
for f in bm.faces:
    c = f.calc_center_median()
    if c.z > 0.6 and abs(c.y) < 0.4:
        f.material_index = ALAS

# Parabrisas: la parte de arriba de la nariz, azul.
for f in bm.faces:
    c = f.calc_center_median()
    if 1.4 < c.x < 3.5 and f.normal.z > 0.25 and abs(c.y) < 0.7:
        f.material_index = CABINA

# Bevel suave en las aristas largas del cuerpo.
largas = [e for e in bm.edges if e.calc_length() > 2.0 and all(abs(v.co.y) < 0.85 and v.co.z < 0.55 for v in e.verts)]
bmesh.ops.bevel(bm, geom=largas, offset=0.05, segments=1, affect="EDGES", profile=0.5)

bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
bm.to_mesh(cubo.data)
bm.free()
cubo.data.update()

import os, math
carpeta = os.getcwd()
bpy.ops.wm.save_as_mainfile(filepath=os.path.join(carpeta, "nave.blend"))
bpy.ops.object.select_all(action="DESELECT")
cubo.select_set(True)
bpy.context.view_layer.objects.active = cubo
bpy.ops.wm.obj_export(
    filepath=os.path.join(carpeta, "..", "assets", "nave.obj"),
    export_selected_objects=True,
    export_triangulated_mesh=True,
    export_materials=True,
    export_uv=False,
    forward_axis="NEGATIVE_Z",
    up_axis="Y",
)

# Vista previa (camara y luz solo para la foto, no se guardan en el .blend).
escena = bpy.context.scene
cam = bpy.data.objects.new("Camara", bpy.data.cameras.new("Camara"))
escena.collection.objects.link(cam)
cam.location = (8.5, -9.0, 6.5)
direccion = Vector((0.4, 0, 0.6)) - cam.location
cam.rotation_euler = direccion.to_track_quat("-Z", "Y").to_euler()
escena.camera = cam
escena.render.engine = "BLENDER_WORKBENCH"
escena.display.shading.color_type = "MATERIAL"
escena.display.shading.show_object_outline = True
escena.render.resolution_x, escena.render.resolution_y = 900, 600
escena.render.filepath = os.path.join(carpeta, "vista_previa.png")
bpy.ops.render.render(write_still=True)
