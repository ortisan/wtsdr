"use client";

import { useForm, useFieldArray } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useState } from "react";

const serviceSchema = z.object({
    businessName: z.string().min(3, "Nome deve ter pelo menos 3 caracteres"),
    description: z.string().min(10, "Descrição deve ter pelo menos 10 caracteres"),
    servicesProvided: z.array(z.string().min(1)).min(1, "Adicione pelo menos um serviço"),
    operatingHours: z.array(
        z.object({
            day: z.string(),
            open: z.string(),
            close: z.string(),
            active: z.boolean(),
        })
    ),
});

type ServiceFormData = z.infer<typeof serviceSchema>;

const DAYS = ["Segunda", "Terça", "Quarta", "Quinta", "Sexta", "Sábado", "Domingo"];

export default function NewServicePage() {
    const [serviceInput, setServiceInput] = useState("");
    const [photos, setPhotos] = useState<File[]>([]);
    const [videos, setVideos] = useState<File[]>([]);

    const {
        register,
        handleSubmit,
        control,
        setValue,
        watch,
        formState: { errors, isSubmitting },
    } = useForm<ServiceFormData>({
        resolver: zodResolver(serviceSchema),
        defaultValues: {
            businessName: "",
            description: "",
            servicesProvided: [],
            operatingHours: DAYS.map((day) => ({
                day,
                open: "08:00",
                close: "18:00",
                active: day !== "Domingo",
            })),
        },
    });

    const servicesProvided = watch("servicesProvided");

    const addService = () => {
        if (serviceInput.trim()) {
            setValue("servicesProvided", [...servicesProvided, serviceInput.trim()]);
            setServiceInput("");
        }
    };

    const removeService = (index: number) => {
        setValue(
            "servicesProvided",
            servicesProvided.filter((_, i) => i !== index)
        );
    };

    const onSubmit = async (data: ServiceFormData) => {
        console.log("Form data:", data);
        console.log("Photos:", photos);
        console.log("Videos:", videos);
        // TODO: Call backend API to create the service
        alert("Serviço cadastrado com sucesso! (mock)");
    };

    return (
        <div className="max-w-2xl">
            <div className="mb-8">
                <h2 className="text-xl font-semibold text-[var(--color-text)]">
                    Cadastrar Novo Serviço
                </h2>
                <p className="text-sm text-[var(--color-text-secondary)] mt-1">
                    Preencha os dados do seu negócio
                </p>
            </div>

            <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                {/* Business Name */}
                <div>
                    <label className="label">Nome da Empresa</label>
                    <input
                        {...register("businessName")}
                        className={`input-field ${errors.businessName ? "input-error" : ""}`}
                        placeholder="Ex: Encanamentos Silva"
                    />
                    {errors.businessName && (
                        <p className="error-text">{errors.businessName.message}</p>
                    )}
                </div>

                {/* Description */}
                <div>
                    <label className="label">Descrição</label>
                    <textarea
                        {...register("description")}
                        rows={3}
                        className={`input-field resize-none ${errors.description ? "input-error" : ""}`}
                        placeholder="Descreva brevemente sua empresa e especialidades"
                    />
                    {errors.description && (
                        <p className="error-text">{errors.description.message}</p>
                    )}
                </div>

                {/* Services Provided - Tag Input */}
                <div>
                    <label className="label">Serviços Prestados</label>
                    <div className="flex gap-2">
                        <input
                            value={serviceInput}
                            onChange={(e) => setServiceInput(e.target.value)}
                            onKeyDown={(e) => {
                                if (e.key === "Enter") {
                                    e.preventDefault();
                                    addService();
                                }
                            }}
                            className="input-field"
                            placeholder="Adicione um serviço e pressione Enter"
                        />
                        <button type="button" onClick={addService} className="btn-outline whitespace-nowrap">
                            Adicionar
                        </button>
                    </div>
                    {errors.servicesProvided && (
                        <p className="error-text">{errors.servicesProvided.message}</p>
                    )}
                    <div className="flex flex-wrap gap-2 mt-3">
                        {servicesProvided.map((service, idx) => (
                            <span
                                key={idx}
                                className="inline-flex items-center gap-1 bg-blue-50 text-[var(--color-primary)] text-xs font-medium px-3 py-1.5 rounded-full"
                            >
                                {service}
                                <button
                                    type="button"
                                    onClick={() => removeService(idx)}
                                    className="ml-1 hover:text-[var(--color-error)]"
                                >
                                    ✕
                                </button>
                            </span>
                        ))}
                    </div>
                </div>

                {/* Operating Hours */}
                <div>
                    <label className="label">Horários de Funcionamento</label>
                    <div className="card p-4 space-y-2">
                        {DAYS.map((day, idx) => (
                            <div key={day} className="flex items-center gap-4 text-sm">
                                <label className="flex items-center gap-2 w-28">
                                    <input
                                        type="checkbox"
                                        {...register(`operatingHours.${idx}.active`)}
                                        className="accent-[var(--color-primary)]"
                                    />
                                    <span className="text-[var(--color-text)]">{day}</span>
                                </label>
                                <input
                                    type="time"
                                    {...register(`operatingHours.${idx}.open`)}
                                    className="input-field !w-auto !py-1.5"
                                />
                                <span className="text-[var(--color-text-secondary)]">até</span>
                                <input
                                    type="time"
                                    {...register(`operatingHours.${idx}.close`)}
                                    className="input-field !w-auto !py-1.5"
                                />
                            </div>
                        ))}
                    </div>
                </div>

                {/* Photos Upload */}
                <div>
                    <label className="label">Fotos</label>
                    <input
                        type="file"
                        accept="image/*"
                        multiple
                        onChange={(e) => setPhotos(Array.from(e.target.files || []))}
                        className="input-field"
                    />
                    {photos.length > 0 && (
                        <p className="text-xs text-[var(--color-text-secondary)] mt-2">
                            {photos.length} foto(s) selecionada(s)
                        </p>
                    )}
                </div>

                {/* Videos Upload */}
                <div>
                    <label className="label">Vídeos</label>
                    <input
                        type="file"
                        accept="video/*"
                        multiple
                        onChange={(e) => setVideos(Array.from(e.target.files || []))}
                        className="input-field"
                    />
                    {videos.length > 0 && (
                        <p className="text-xs text-[var(--color-text-secondary)] mt-2">
                            {videos.length} vídeo(s) selecionado(s)
                        </p>
                    )}
                </div>

                {/* Submit */}
                <div className="pt-4">
                    <button type="submit" disabled={isSubmitting} className="btn-primary w-full">
                        {isSubmitting ? "Salvando..." : "Cadastrar Serviço"}
                    </button>
                </div>
            </form>
        </div>
    );
}
